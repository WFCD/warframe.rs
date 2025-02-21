#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
//! Provides a client that acts as the baseline for interacting with the market API

use std::{
    sync::Arc,
    time::Duration,
};

use super::{
    error::ApiError,
    models::{
        item::Item,
        item_info::ItemInfo,
        orders::Order,
        statistic_item::{
            StatisticItem,
            StatisticItemPayload,
        },
    },
};

#[cfg(feature = "market_cache")]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[doc = "A cached value"]
pub enum CacheValue {
    /// `StatisticItem`
    StatisticItem(Arc<StatisticItem>),
    /// `ItemInfo`
    ItemInfo(Arc<ItemInfo>),
    /// Items
    Items(Arc<Vec<Item>>),
    /// Orders
    Orders(Arc<Vec<Order>>),
}

/// The client
#[derive(Debug, Clone)]
#[cfg_attr(not(feature = "market_cache"), derive(Default))]
pub struct Client {
    session: reqwest::Client,
    #[cfg(feature = "market_cache")]
    cache: moka::future::Cache<String, CacheValue>,
}

impl Client {
    /// Creates a new [Client]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(not(feature = "market_cache"))]
impl Client {
    /// Fetches the statistics of an item via its url_name
    pub async fn item_statistics(&self, item_url: &str) -> Result<StatisticItem, ApiError> {
        let response = self
            .session
            .get(format!(
                "https://api.warframe.market/v1/items/{item_url}/statistics"
            ))
            .send()
            .await?;

        if response.status().is_success() {
            let json_result = response.json::<StatisticItemPayload>().await?;
            Ok(json_result.payload)
        } else {
            Err(response.status().into())
        }
    }

    /// Fetches info about an item via its url_name
    pub async fn item_info(&self, item_url: &str) -> Result<ItemInfo, ApiError> {
        let response = self
            .session
            .get(format!("https://api.warframe.market/v1/items/{item_url}"))
            .send()
            .await?;

        if response.status().is_success() {
            let json_result = response
                .json::<crate::market::models::ItemInfoPayload>()
                .await?;
            Ok(json_result.payload.item)
        } else {
            Err(response.status().into())
        }
    }

    /// Fetches all tradable items
    pub async fn items(&self) -> Result<Vec<Item>, ApiError> {
        let response = self
            .session
            .get("https://api.warframe.market/v1/items")
            .send()
            .await?;

        if response.status().is_success() {
            let json_result = response
                .json::<crate::market::models::ItemsPayload>()
                .await?;
            Ok(json_result.payload.items)
        } else {
            Err(response.status().into())
        }
    }

    /// Fetches all orders of a specific item via its url_name
    pub async fn orders(&self, item_url: &str) -> Result<Vec<Order>, ApiError> {
        let response = self
            .session
            .get(format!(
                "https://api.warframe.market/v1/items/{item_url}/orders"
            ))
            .send()
            .await?;

        if response.status().is_success() {
            let json_result = response
                .json::<crate::market::models::OrderPayload>()
                .await?;
            Ok(json_result.payload.orders)
        } else {
            Err(response.status().into())
        }
    }
}

/// The cached version of the client
#[cfg(feature = "market_cache")]
pub mod cached {
    use std::sync::Arc;

    pub use moka;
    use moka::future::Cache;
    use reqwest::Response;

    use super::{
        ApiError,
        CacheValue,
        Client,
        Duration,
        Item,
        ItemInfo,
        Order,
        StatisticItem,
        StatisticItemPayload,
    };
    use crate::market::models::{
        item::ItemsPayload,
        item_info::ItemInfoPayload,
        orders::OrderPayload,
    };

    /// Whether an item has been gotten via a cache hit or freshly fetched.
    pub enum FetchResult {
        /// Cache hit
        Cached(CacheValue),
        /// Fetched
        Fetched(Result<Response, ApiError>),
    }

    #[cfg(feature = "market_cache")]
    impl Client {
        /// Creates a new client with a custom cache
        #[must_use]
        pub fn new_with_cache(cache: Cache<String, CacheValue>) -> Self {
            Self {
                session: reqwest::Client::default(),
                cache,
            }
        }

        async fn get_cached_or_new(&self, url: &str) -> FetchResult {
            if let Some(value) = self.cache.get(url).await {
                FetchResult::Cached(value)
            } else {
                FetchResult::Fetched(self.session.get(url).send().await.map_err(ApiError::from))
            }
        }

        /// Fetches the statistics of an item via its `url_name`
        pub async fn item_statistics(
            &self,
            item_url: &str,
        ) -> Result<Arc<StatisticItem>, ApiError> {
            match self
                .get_cached_or_new(&format!(
                    "https://api.warframe.market/v1/items/{item_url}/statistics"
                ))
                .await
            {
                FetchResult::Cached(value) => {
                    if let CacheValue::StatisticItem(item) = value {
                        Ok(item)
                    } else {
                        panic!("FATAL: Wrong cache insertion was made!") // TODO: Improve this error
                                                                         // msg
                    }
                }
                FetchResult::Fetched(response) => {
                    let response = response?;
                    if response.status().is_success() {
                        let url = response.url().to_string();
                        let json_result = response.json::<StatisticItemPayload>().await?;

                        let item = Arc::new(json_result.payload);
                        self.cache
                            .insert(url, CacheValue::StatisticItem(item.clone()))
                            .await;
                        Ok(item)
                    } else {
                        Err(response.status().into())
                    }
                }
            }
        }

        /// Fetches info about an item via its `url_name`
        pub async fn item_info(&self, item_url: &str) -> Result<Arc<ItemInfo>, ApiError> {
            match self
                .get_cached_or_new(&format!("https://api.warframe.market/v1/items/{item_url}"))
                .await
            {
                FetchResult::Cached(value) => {
                    if let CacheValue::ItemInfo(item) = value {
                        Ok(item)
                    } else {
                        panic!("FATAL: Wrong cache insertion was made!") // TODO: Improve this error
                                                                         // msg
                    }
                }
                FetchResult::Fetched(response) => {
                    let response = response?;
                    if response.status().is_success() {
                        let url = response.url().to_string();
                        let json_result = response.json::<ItemInfoPayload>().await?;

                        let item = Arc::new(json_result.payload.item);
                        self.cache
                            .insert(url, CacheValue::ItemInfo(item.clone()))
                            .await;
                        Ok(item)
                    } else {
                        Err(response.status().into())
                    }
                }
            }
        }

        /// Fetches all tradable items
        pub async fn items(&self) -> Result<Arc<Vec<Item>>, ApiError> {
            match self
                .get_cached_or_new("https://api.warframe.market/v1/items")
                .await
            {
                FetchResult::Cached(value) => {
                    if let CacheValue::Items(item) = value {
                        Ok(item)
                    } else {
                        panic!("FATAL: Wrong cache insertion was made!") // TODO: Improve this error
                                                                         // msg
                    }
                }
                FetchResult::Fetched(response) => {
                    let response = response?;
                    if response.status().is_success() {
                        let url = response.url().to_string();
                        let json_result = response.json::<ItemsPayload>().await?;

                        let item = Arc::new(json_result.payload.items);
                        self.cache
                            .insert(url, CacheValue::Items(item.clone()))
                            .await;
                        Ok(item)
                    } else {
                        Err(response.status().into())
                    }
                }
            }
        }

        /// Fetches all orders of a specific item via its `url_name`
        pub async fn orders(&self, item_url: &str) -> Result<Arc<Vec<Order>>, ApiError> {
            match self
                .get_cached_or_new(&format!(
                    "https://api.warframe.market/v1/items/{item_url}/orders"
                ))
                .await
            {
                FetchResult::Cached(value) => {
                    if let CacheValue::Orders(item) = value {
                        Ok(item)
                    } else {
                        panic!("FATAL: Wrong cache insertion was made!") // TODO: Improve this error
                                                                         // msg
                    }
                }
                FetchResult::Fetched(response) => {
                    let response = response?;
                    if response.status().is_success() {
                        let url = response.url().to_string();
                        let json_result = response.json::<OrderPayload>().await?;

                        let item = Arc::new(json_result.payload.orders);
                        self.cache
                            .insert(url, CacheValue::Orders(item.clone()))
                            .await;
                        Ok(item)
                    } else {
                        Err(response.status().into())
                    }
                }
            }
        }
    }

    #[cfg(feature = "market_cache")]
    impl Default for Client {
        fn default() -> Self {
            Self {
                session: reqwest::Client::default(),
                cache: Cache::builder()
                    .max_capacity(10_000)
                    .time_to_live(Duration::from_secs(1800))
                    .name("warframe_market_cache")
                    .build(),
            }
        }
    }
}
