use derive_builder::Builder;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
#[cfg(feature = "market_cache")]
use {
    super::ItemShort,
    super::cache::{
        CacheKey,
        SlugContext,
        Slugs,
    },
    super::queryable::LichWeapon,
    crate::market::{
        models::{
            lich_ephemera::LichEphemera,
            sister_ephemera::SisterEphemera,
            sister_weapon::SisterWeapon,
        },
        queryable::{
            Location,
            Mission,
            Npc,
        },
    },
    moka::future::Cache,
    std::any::type_name,
    std::collections::HashSet,
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
    queryable::Riven,
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
            .max_capacity(12) // 1 for each language
            .build()
    )]
    items_cache: Cache<Language, Arc<[ItemShort]>>,

    #[cfg(feature = "market_cache")]
    #[builder(
        default = Cache::builder()
            .time_to_live(Duration::from_secs(86400))
            .max_capacity(7) // 1 for each slug category
            .build()
    )]
    slug_cache: Cache<SlugContext, Slugs>,
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
        ClientBuilder::default()
            .build()
            .expect("default client builder should never fail")
    }

    #[must_use]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    #[cfg(feature = "market_cache")]
    async fn get_from_cache<T>(&self, key: &CacheKey) -> Option<T>
    where
        T: 'static + Send + Sync + Clone,
    {
        if let Some(item) = self
            .cache
            .get(key)
            .await
            .and_then(|item| item.downcast_ref::<T>().cloned())
        {
            tracing::debug!("cache hit for {key:?}");
            return Some(item);
        }

        None
    }

    #[cfg(feature = "market_cache")]
    async fn insert_into_cache<T>(&self, key: CacheKey, data: T)
    where
        T: 'static + Send + Sync + Clone,
    {
        tracing::debug!("cache insertion for {key:?}");
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
    pub async fn fetch_item(&self, item_slug: &impl AsRef<str>) -> Result<Option<Item>> {
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
        #[cfg(feature = "market_cache")]
        let key = CacheKey::new(language, T::ENDPOINT);

        #[cfg(feature = "market_cache")]
        if let Some(data) = self.get_from_cache::<T::Data>(&key).await {
            return Ok(data);
        }

        ratelimit!(self);

        let data = T::query(&self.client, language).await?;

        #[cfg(feature = "market_cache")]
        self.insert_into_cache(key, data.clone()).await;

        Ok(data)
    }

    /// Fetches an item by its slug.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails or if the API returns an error.
    pub async fn fetch_item_using_language(
        &self,
        slug: &impl AsRef<str>,
        language: Language,
    ) -> Result<Option<Item>> {
        let endpoint = format!("/item/{}", slug.as_ref());

        self.try_get_item(&endpoint, language).await
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
    pub async fn set_items_of(
        &self,
        slug: &impl AsRef<str>,
        language: Language,
    ) -> Result<Option<SetItems>> {
        let endpoint = format!("/item/{}/set", slug.as_ref());

        self.try_get_item(&endpoint, language).await
    }

    /// Fetches a riven item by its slug.
    ///
    /// # Errors
    /// This function will return an error if the request fails or if the API returns an error.
    pub async fn fetch_riven_item(
        &self,
        slug: &impl AsRef<str>,
        language: Language,
    ) -> Result<Option<Riven>> {
        let endpoint = format!("/riven/weapon/{}", slug.as_ref());

        self.try_get_item(&endpoint, language).await
    }

    async fn try_get_item<T>(&self, endpoint: &str, language: Language) -> Result<Option<T>>
    where
        T: Send + Sync + Clone + DeserializeOwned + 'static,
    {
        #[cfg(feature = "market_cache")]
        let key = CacheKey::new(language, endpoint);

        #[cfg(feature = "market_cache")]
        if let Some(data) = self.get_from_cache::<T>(&key).await {
            tracing::debug!(
                "cache hit for {} with language `{}`",
                type_name::<T>(),
                language
            );
            return Ok(Some(data));
        }

        ratelimit!(self);

        let response = self.fetch_from_api(endpoint, language).await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let item = response.json::<ResponseBase<T>>().await?;
        match item.data {
            Some(data) => {
                #[cfg(feature = "market_cache")]
                {
                    tracing::debug!(
                        "cache insertion for {} with language `{}`",
                        type_name::<T>(),
                        language
                    );
                    self.insert_into_cache(key, data.clone()).await;
                }

                Ok(Some(data))
            }
            None => Err(Error::Api(item.error.ok_or(Error::EmptyErrorAndData)?)),
        }
    }

    /// Returns all available items on warframe.market.
    ///
    /// # Errors
    /// See [Error](crate::market::error::Error) for more information.
    #[cfg(feature = "market_cache")]
    pub async fn items(&self, language: Language) -> Result<Arc<[ItemShort]>> {
        #[cfg(feature = "market_cache")]
        if let Some(data) = self.items_cache.get(&language).await {
            tracing::debug!("cache hit for items with language `{:?}`", language);
            return Ok(data);
        }

        ratelimit!(self);

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

    #[cfg(feature = "market_cache")]
    async fn get_slugs(&self, context: SlugContext) -> Result<Slugs> {
        if let Some(data) = self.slug_cache.get(&context).await {
            tracing::debug!("cache hit for slugs");
            return Ok(data);
        }

        let slugs = match context {
            SlugContext::Items => self
                .items(Language::En)
                .await?
                .iter()
                .map(|item| item.slug.clone())
                .collect::<HashSet<_>>(),

            SlugContext::Rivens => to_hashset!(self, Riven),
            SlugContext::LichWeapons => to_hashset!(self, LichWeapon),
            SlugContext::LichEphemeras => to_hashset!(self, LichEphemera),
            SlugContext::SisterWeapons => to_hashset!(self, SisterWeapon),
            SlugContext::SisterEphemeras => to_hashset!(self, SisterEphemera),
            SlugContext::Locations => to_hashset!(self, Location),
            SlugContext::Npcs => to_hashset!(self, Npc),
            SlugContext::Missions => to_hashset!(self, Mission),
        };

        let slugs = Arc::new(slugs);

        tracing::debug!("cache insertion for slugs");
        self.slug_cache.insert(context, Arc::clone(&slugs)).await;

        Ok(slugs)
    }

    /// Why is this async?
    ///
    /// -> It depends on the underlying cache for items. As the fetching is async, this function has
    /// to be async as well.
    ///
    /// IMPORTANT NOTE:
    /// Slug validity is dependant on the context. For example, general weapon slugs are likely only
    /// valid for the [`SlugCategory::Rivens`].
    ///
    /// # Errors
    /// Whenever [items](crate::market::client::Client::items) errors.
    #[cfg(feature = "market_cache")]
    pub async fn is_slug_valid(
        &self,
        context: SlugContext,
        slug: &impl AsRef<str>,
    ) -> Result<bool> {
        Ok(self.get_slugs(context).await?.contains(slug.as_ref()))
    }

    /// Invalidates the items cache and all dependant caches (mainly the slug cache)
    #[cfg(feature = "market_cache")]
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

#[cfg(feature = "market_cache")]
macro_rules! to_hashset {
    ($self:expr, $ty:ty) => {
        $self
            .fetch::<$ty>()
            .await?
            .iter()
            .map(|item| item.slug.clone())
            .collect::<HashSet<_>>()
    };
}

#[cfg(feature = "market_cache")]
use to_hashset;
