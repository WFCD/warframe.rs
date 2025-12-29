#![allow(clippy::missing_errors_doc)]

//! A client to do all sorts of things with the API

use std::{
    any::{
        Any,
        TypeId,
        type_name,
    },
    sync::Arc,
    time::Duration,
};

use moka::future::Cache;
use reqwest::StatusCode;

use super::{
    Queryable,
    error::Error,
    language::Language,
    models::items::Item,
};

#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// The time a nested listener should sleep before making another request
    pub nested_listener_sleep: Duration,

    /// The time a listener sleeps upon reaching it's expiry until it tries to fetch the updated
    /// data
    pub listener_sleep_timeout: Duration,

    /// Whether the items cache should create entries of items not found by the API.
    /// # Advantage
    /// If the same mistake happens multiple times, only a single request will be sent
    ///
    /// # Disadvantage
    /// Can bloat memory usage.
    pub cache_404_item_requests: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            nested_listener_sleep: Duration::from_mins(5),
            listener_sleep_timeout: Duration::from_mins(5),
            cache_404_item_requests: false,
        }
    }
}

type TypeCache = Cache<(Language, TypeId), Arc<dyn Any + Send + Sync>>;
type ItemCache = Cache<(Language, Box<str>), Option<Item>>;

/// The client that acts as a convenient way to query models.
///
/// ## Example
/// ```rust
/// use warframe::worldstate::{
///     Client,
///     Error,
///     queryable::{
///         Cetus,
///         Fissure,
///     },
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     let client = Client::default();
///
///     let cetus: Cetus = client.fetch::<Cetus>().await?;
///     let fissures: Vec<Fissure> = client.fetch::<Fissure>().await?;
///
///     Ok(())
/// }
/// ```
///
/// Check the [queryable](crate::worldstate::queryable) module for all queryable types.
#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) config: ClientConfig,
    type_cache: TypeCache,
    items_cache: ItemCache,
}

impl Default for Client {
    /// Creates a new [Client].
    ///
    /// Requests are sent to `https://api.warframestat.us/` by default.
    fn default() -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: "https://api.warframestat.us".to_owned(),
            config: ClientConfig::default(),
            type_cache: Cache::builder()
                .time_to_live(Duration::from_mins(5))
                .build(),
            items_cache: Cache::builder()
                .time_to_live(Duration::from_hours(12))
                .build(),
        }
    }
}

impl Client {
    /// Creates a new [Client] with the option to supply a custom reqwest client and a base url.
    #[must_use]
    pub fn new(
        reqwest_client: reqwest::Client,
        base_url: String,
        config: ClientConfig,
        type_cache: TypeCache,
        items_cache: ItemCache,
    ) -> Self {
        Self {
            http: reqwest_client,
            base_url,
            config,
            type_cache,
            items_cache,
        }
    }

    async fn type_cached<T, F>(&self, language: Language, fallback: F) -> Result<T::Return, Error>
    where
        T: Queryable,
        F: AsyncFn() -> Result<T::Return, Error>,
    {
        let type_id = TypeId::of::<T::Return>();

        if let Some(item) = self
            .type_cache
            .get(&(language, type_id))
            .await
            .and_then(|any| any.downcast_ref::<T::Return>().cloned())
        {
            tracing::debug!("cache hit for type {}", type_name::<T::Return>());
            return Ok(item);
        }

        let item = fallback().await?;

        self.type_cache
            .insert((language, type_id), Arc::new(item.clone()))
            .await;

        Ok(item)
    }

    /// Fetches an instance of a specified model.
    ///
    /// # Example
    /// ```rust,no_run
    /// use warframe::worldstate::{
    ///     Client,
    ///     Error,
    ///     queryable::{
    ///         Cetus,
    ///         Fissure,
    ///     },
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     let cetus: Cetus = client.fetch::<Cetus>().await?;
    ///     let fissures: Vec<Fissure> = client.fetch::<Fissure>().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn fetch<T>(&self) -> Result<T::Return, Error>
    where
        T: Queryable,
    {
        self.type_cached::<T, _>(Language::EN, || T::query(&self.base_url, &self.http))
            .await
    }

    /// Fetches an instance of a specified model in a supplied Language.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use warframe::worldstate::{
    ///     Client,
    ///     Error,
    ///     Language,
    ///     queryable::{
    ///         Cetus,
    ///         Fissure,
    ///     },
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     let cetus: Cetus = client.fetch_using_lang::<Cetus>(Language::ZH).await?;
    ///     let fissures: Vec<Fissure> = client.fetch_using_lang::<Fissure>(Language::ZH).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn fetch_using_lang<T>(&self, language: Language) -> Result<T::Return, Error>
    where
        T: Queryable,
    {
        self.type_cached::<T, _>(language, || {
            T::query_with_language(&self.base_url, &self.http, language)
        })
        .await
    }

    async fn cached_item<F>(
        &self,
        language: Language,
        query: &str,
        fallback: F,
    ) -> Result<Option<Item>, Error>
    where
        F: AsyncFn() -> Result<Option<Item>, Error>,
    {
        let key = (language, Box::from(query));
        if let Some(item) = self.items_cache.get(&key).await {
            tracing::debug!("cache hit for {key:?}");
            return Ok(item);
        }

        let maybe_item = fallback().await?;

        if maybe_item.is_some() || self.config.cache_404_item_requests {
            self.items_cache.insert(key, maybe_item.clone()).await;
        }

        Ok(maybe_item)
    }

    /// Queries an item by its name and returns the closest matching item.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use warframe::worldstate::{
    ///     Client,
    ///     Error,
    ///     items::{
    ///         Item,
    ///         weapon::Weapon,
    ///     },
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     let weapon = client.query_item("Acceltra Prime").await?;
    ///
    ///     assert!(match weapon {
    ///         Some(Item::Weapon(weapon)) => matches!(*weapon, Weapon::Rifle(_)),
    ///         _ => false,
    ///     });
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn query_item(&self, query: &str) -> Result<Option<Item>, Error> {
        self.cached_item(Language::EN, query, || {
            self.query_by_url(format!(
                "{}/items/{}/?language=en",
                self.base_url,
                urlencoding::encode(query),
            ))
        })
        .await
    }

    /// Queries an item by its name and returns the closest matching item.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use warframe::worldstate::{
    ///     Client,
    ///     Error,
    ///     Language,
    ///     items::Item,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     let nano_spores = client
    ///         .query_item_using_lang("Nanosporen", Language::DE)
    ///         .await?;
    ///
    ///     assert!(matches!(nano_spores, Some(Item::Misc(_))));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn query_item_using_lang(
        &self,
        query: &str,
        language: Language,
    ) -> Result<Option<Item>, Error> {
        self.cached_item(language, query, || {
            self.query_by_url(format!(
                "{}/items/{}/?language={}",
                self.base_url,
                urlencoding::encode(query),
                language
            ))
        })
        .await
    }

    async fn query_by_url(&self, url: String) -> Result<Option<Item>, Error> {
        let response = self.http.get(url).send().await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let json = response.text().await?;

        let item = serde_json::from_str::<Item>(&json)?;

        Ok(Some(item))
    }
}
