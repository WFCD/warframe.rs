#![allow(clippy::missing_errors_doc)]

//! A client to do all sorts of things with the API

use std::time::Duration;

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
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            nested_listener_sleep: Duration::from_mins(5),
            listener_sleep_timeout: Duration::from_mins(5),
        }
    }
}

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
}

impl Default for Client {
    /// Creates a new [Client].
    ///
    /// Requests are sent to `https://api.warframestat.us/` by default.
    fn default() -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: "https://api.warframestat.us".to_string(),
            config: ClientConfig::default(),
        }
    }
}

impl Client {
    /// Creates a new [Client] with the option to supply a custom reqwest client and a base url.
    #[must_use]
    pub fn new(reqwest_client: reqwest::Client, base_url: String, config: ClientConfig) -> Self {
        Self {
            http: reqwest_client,
            base_url,
            config,
        }
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
        <T as Queryable>::query(&self.base_url, &self.http).await
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
        T::query_with_language(&self.base_url, &self.http, language).await
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
        self.query_by_url(format!(
            "{}/items/{}/?language=en",
            self.base_url,
            urlencoding::encode(query),
        ))
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
        self.query_by_url(format!(
            "{}/items/{}/?language={}",
            self.base_url,
            urlencoding::encode(query),
            language
        ))
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
