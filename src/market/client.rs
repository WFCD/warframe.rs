use std::{
    any::Any,
    num::NonZeroU32,
    sync::Arc,
    time::Duration,
};

#[cfg(feature = "market_ratelimit")]
use governor::{
    DefaultDirectRateLimiter,
    Quota,
};
#[cfg(feature = "market_cache")]
use moka::future::Cache;
use reqwest::StatusCode;

#[cfg(feature = "market_cache")]
use super::cache::CacheKey;
use super::{
    Error,
    Queryable,
    ResponseBase,
    Result,
    models::{
        item::Item,
        set_items::SetItems,
    },
};
use crate::market::models::i18n::Language;

type StdResult<T, E> = std::result::Result<T, E>;

pub struct Client {
    client: reqwest::Client,

    #[cfg(feature = "market_ratelimit")]
    ratelimiter: DefaultDirectRateLimiter,

    #[cfg(feature = "market_cache")]
    cache: Cache<CacheKey, Arc<dyn Any + Send + Sync>>,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    #[must_use]
    #[cfg_attr(
        feature = "market_ratelimit",
        expect(
            clippy::missing_panics_doc,
            reason = "NonZeroU32::new(3) is guaranteed to succeed"
        )
    )]
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),

            #[cfg(feature = "market_ratelimit")]
            ratelimiter: DefaultDirectRateLimiter::direct(Quota::per_second(
                NonZeroU32::new(3).unwrap(),
            )),

            #[cfg(feature = "market_cache")]
            cache: Cache::builder()
                .time_to_live(Duration::from_secs(600))
                .max_capacity(1000)
                .build(),
        }
    }

    #[must_use]
    #[cfg_attr(
        feature = "market_ratelimit",
        expect(
            clippy::missing_panics_doc,
            reason = "NonZeroU32::new(3) is guaranteed to succeed"
        )
    )]
    pub fn new_with_cache_duration(duration: Duration) -> Self {
        Self {
            client: reqwest::Client::new(),

            #[cfg(feature = "market_ratelimit")]
            ratelimiter: DefaultDirectRateLimiter::direct(Quota::per_second(
                NonZeroU32::new(3).unwrap(),
            )),

            #[cfg(feature = "market_cache")]
            cache: Cache::builder()
                .time_to_live(duration)
                .max_capacity(1000)
                .build(),
        }
    }

    #[cfg(feature = "market_cache")]
    async fn get_from_cache<T>(&self, key: CacheKey) -> Option<T>
    where
        T: 'static + Send + Sync + Clone,
    {
        self.cache
            .get(&key)
            .await
            .and_then(|item| item.downcast_ref::<T>().cloned())
    }

    #[cfg(feature = "market_cache")]
    async fn insert_into_cache<T>(&self, key: CacheKey, data: T)
    where
        T: 'static + Send + Sync + Clone,
    {
        self.cache.insert(key, Arc::new(data)).await;
    }

    async fn fetch_from_api(
        &self,
        url: &str,
        language: Language,
    ) -> StdResult<reqwest::Response, reqwest::Error> {
        self.client
            .get(url)
            .header("Language", language.to_string())
            .send()
            .await
    }

    /// Fetches the data of a queryable model.
    pub async fn fetch<T>(&self) -> Result<T::Data>
    where
        T: Queryable,
        T::Data: Send + Sync + 'static,
    {
        self.fetch_using_language::<T>(Language::En).await
    }

    /// Fetches an item by its slug.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails or if the API returns an error.
    pub async fn fetch_item(&self, item_slug: &str) -> Result<Option<Item>> {
        self.fetch_item_using_language(item_slug, Language::En)
            .await
    }

    /// Fetches the data of a queryable model.
    /// This function allows you to specify the language to use.
    ///
    /// Translations can be found in the i18n fields.
    pub async fn fetch_using_language<T>(&self, language: Language) -> Result<T::Data>
    where
        T: Queryable,
        T::Data: Send + Sync + 'static,
    {
        try_get_cache!(self, T::Data, CacheKey::language::<T::Data>(language));

        ratelimit!(self);

        let data = T::query(&self.client, language).await?;

        insert_cache!(self, CacheKey::language::<T::Data>(language), data.clone());

        Ok(data)
    }

    /// Fetches an item by its slug.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails or if the API returns an error.
    pub async fn fetch_item_using_language(
        &self,
        slug: &str,
        language: Language,
    ) -> Result<Option<Item>> {
        try_get_cache!(option self, Item, CacheKey::new::<Item>(language, slug));

        ratelimit!(self);

        let response = self
            .fetch_from_api(
                format!("https://api.warframe.market/v2/item/{slug}").as_str(),
                language,
            )
            .await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let item = response.json::<ResponseBase<Item>>().await?;

        if let Some(error) = item.error {
            return Err(Error::Api(error));
        }

        let item = item.data.ok_or(Error::EmptyErrorAndData)?;

        insert_cache!(self, CacheKey::new::<Item>(language, slug), item.clone());

        Ok(Some(item))
    }

    /// Retrieve Information on Item Sets
    /// In WFM, items can either be standalone or part of a set. A set is a collection of related
    /// items that are often traded together.
    ///
    /// - If the queried item is not part of any set, the response will contain an array with just
    ///   that one item.
    /// - If the item is part of a set or is a set itself, the response will include an array of all
    ///   items within that set.
    ///
    ///
    /// # Errors
    /// See [Error](crate::market::error::Error) for more information.
    pub async fn set_items_of(&self, slug: &str, language: Language) -> Result<Option<SetItems>> {
        try_get_cache!(option self, SetItems, CacheKey::new::<SetItems>(language, slug));

        ratelimit!(self);

        let response = self
            .fetch_from_api(
                format!("https://api.warframe.market/v2/item/{slug}/set").as_str(),
                language,
            )
            .await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let item = response.json::<ResponseBase<SetItems>>().await?;

        if let Some(error) = item.error {
            return Err(Error::Api(error));
        }

        let item = item.data.ok_or(Error::EmptyErrorAndData)?;

        insert_cache!(
            self,
            CacheKey::new::<SetItems>(language, slug),
            item.clone()
        );

        Ok(Some(item))
    }
}

macro_rules! ratelimit {
    ($self:expr) => {
        #[cfg(feature = "market_ratelimit")]
        $self.ratelimiter.until_ready().await;
    };
}

use ratelimit;

macro_rules! try_get_cache {
    ($self:expr, $ty:ty, $key:expr) => {
        #[cfg(feature = "market_cache")]
        let __key = $key;

        #[cfg(feature = "market_cache")]
        if let Some(data) = $self.get_from_cache::<$ty>(__key.clone()).await {
            return Ok(data);
        }
    };

    (option $self:expr, $ty:ty, $key:expr) => {
        #[cfg(feature = "market_cache")]
        let __key = $key;

        #[cfg(feature = "market_cache")]
        if let Some(data) = $self.get_from_cache::<$ty>(__key.clone()).await {
            return Ok(Some(data));
        }
    };
}

use try_get_cache;

macro_rules! insert_cache {
    ($self:expr, $key:expr, $data:expr) => {
        #[cfg(feature = "market_cache")]
        $self.insert_into_cache($key, $data).await;
    };
}

use insert_cache;
