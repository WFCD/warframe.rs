use std::collections::HashSet;

use derive_builder::Builder;
use reqwest::StatusCode;
#[cfg(feature = "market_cache")]
use {
    super::cache::CacheKey,
    super::queryable::ItemShort,
    moka::future::Cache,
    std::{
        any::Any,
        sync::Arc,
        time::Duration,
    },
};
#[cfg(feature = "market_ratelimit")]
use {
    governor::{
        DefaultDirectRateLimiter,
        Quota,
    },
    std::num::NonZeroU32,
};

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
use crate::market::{
    BASE_URL,
    models::i18n::Language,
};

type StdResult<T, E> = std::result::Result<T, E>;

#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct Client {
    #[builder(default)]
    client: reqwest::Client,

    #[cfg(feature = "market_ratelimit")]
    #[builder(
        setter(skip),
        default = DefaultDirectRateLimiter::direct(Quota::per_second(
            NonZeroU32::new(3).unwrap(),
        ))
    )]
    ratelimiter: DefaultDirectRateLimiter,

    #[cfg(feature = "market_cache")]
    #[builder(
        default = Cache::builder()
            .time_to_live(Duration::from_secs(600))
            .max_capacity(1000)
            .build()
    )]
    cache: Cache<CacheKey, Arc<dyn Any + Send + Sync>>,

    #[cfg(feature = "market_cache")]
    #[builder(
        default = Cache::builder()
            .time_to_live(Duration::from_secs(86400))
            .max_capacity(1000)
            .build()
    )]
    items_cache: Cache<Language, Arc<[ItemShort]>>,

    #[cfg(feature = "market_cache")]
    #[builder(
        default = Cache::builder()
            .time_to_live(Duration::from_secs(86400))
            .max_capacity(1000)
            .build()
    )]
    slug_cache: Cache<(), Arc<HashSet<String>>>,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new() -> Self {
        ClientBuilder::default().build().unwrap()
    }

    #[must_use]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
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
        endpoint: &str,
        language: Language,
    ) -> StdResult<reqwest::Response, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}{endpoint}"))
            .header("Language", language.to_string())
            .send()
            .await
    }

    /// Fetches the data of a queryable model.
    pub async fn fetch<T>(&self) -> Result<T::Data>
    where
        T: Queryable,
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
    {
        try_get_cache!(self, T::Data, CacheKey::new(language, T::ENDPOINT));

        ratelimit!(self);

        let data = T::query(&self.client, language).await?;

        insert_cache!(self, CacheKey::new(language, T::ENDPOINT), data.clone());

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
        let endpoint = format!("/item/{slug}");

        try_get_cache!(option self, Item, CacheKey::new(language, &endpoint));

        ratelimit!(self);

        let response = self.fetch_from_api(&endpoint, language).await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let item = response.json::<ResponseBase<Item>>().await?;

        if let Some(error) = item.error {
            return Err(Error::Api(error));
        }

        let item = item.data.ok_or(Error::EmptyErrorAndData)?;

        insert_cache!(self, CacheKey::new(language, &endpoint), item.clone());

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
        let endpoint = format!("/item/{slug}/set");

        try_get_cache!(option self, SetItems, CacheKey::new(language, &endpoint));

        ratelimit!(self);

        let response = self.fetch_from_api(&endpoint, language).await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let item = response.json::<ResponseBase<SetItems>>().await?;

        if let Some(error) = item.error {
            return Err(Error::Api(error));
        }

        let item = item.data.ok_or(Error::EmptyErrorAndData)?;

        insert_cache!(self, CacheKey::new(language, &endpoint), item.clone());

        Ok(Some(item))
    }

    /// Returns all available items on warframe.market.
    ///
    /// # Errors
    /// See [Error](crate::market::error::Error) for more information.
    pub async fn items(&self, language: Language) -> Result<Arc<[ItemShort]>> {
        #[cfg(feature = "market_cache")]
        if let Some(data) = self.items_cache.get(&language).await {
            tracing::debug!("cache hit for items with language `{:?}`", language);
            return Ok(data);
        }

        let response = self.fetch_from_api("/items", language).await?;

        let response_base = response.json::<ResponseBase<Vec<ItemShort>>>().await?;

        if let Some(error) = response_base.error {
            return Err(Error::Api(error));
        }

        let items: Arc<[ItemShort]> = response_base.data.ok_or(Error::EmptyErrorAndData)?.into();

        #[cfg(feature = "market_cache")]
        {
            tracing::debug!("cache insertion for items with language `{:?}`", language);
            self.items_cache.insert(language, Arc::clone(&items)).await;
        }

        Ok(items)
    }

    async fn get_slugs(&self) -> Result<Arc<HashSet<String>>> {
        #[cfg(feature = "market_cache")]
        if let Some(data) = self.slug_cache.get(&()).await {
            tracing::debug!("cache hit for slugs");
            return Ok(data);
        }

        let items = Arc::new(
            self.items(Language::En)
                .await?
                .iter()
                .map(|item| item.slug.clone())
                .collect::<HashSet<_>>(),
        );

        #[cfg(feature = "market_cache")]
        {
            tracing::debug!("cache insertion for slugs");
            self.slug_cache.insert((), Arc::clone(&items)).await;
        }

        Ok(items)
    }

    /// Why is this async?
    ///
    /// It depends on the underlying cache for items. As the fetching is async, this function has to
    /// be async as well.
    ///
    /// # Errors
    /// Whenever [items](crate::market::client::Client::items) errors.
    pub async fn is_slug_valid(&self, slug: &str) -> Result<bool> {
        Ok(self.get_slugs().await?.contains(slug))
    }

    /// Invalidates the items cache and all dependant caches (mainly the slug cache)
    pub fn invalidate_items(&self) {
        self.items_cache.invalidate_all();
        self.slug_cache.invalidate_all();
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
            tracing::debug!("cache hit for {__key:?}",);
            return Ok(data);
        }
    };

    (option $self:expr, $ty:ty, $key:expr) => {
        #[cfg(feature = "market_cache")]
        let __key = $key;

        #[cfg(feature = "market_cache")]
        if let Some(data) = $self.get_from_cache::<$ty>(__key.clone()).await {
            tracing::debug!("cache hit for {__key:?}",);
            return Ok(Some(data));
        }
    };
}

use try_get_cache;

macro_rules! insert_cache {
    ($self:expr, $key:expr, $data:expr) => {
        #[cfg(feature = "market_cache")]
        {
            tracing::debug!("cache insertion for {:?}", $key);
            $self.insert_into_cache($key, $data).await;
        }
    };
}

use insert_cache;
