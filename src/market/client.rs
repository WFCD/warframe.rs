use std::{
    any::{
        Any,
        TypeId,
        type_name,
    },
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

use super::{
    Error,
    Queryable,
    ResponseBase,
    models::item::Item,
};

pub struct Client {
    client: reqwest::Client,

    #[cfg(feature = "market_ratelimit")]
    ratelimiter: DefaultDirectRateLimiter,

    #[cfg(feature = "market_cache")]
    cache: Cache<TypeId, Arc<dyn Any + Send + Sync>>,
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

    #[cfg(feature = "market_ratelimit")]
    async fn ratelimit(&self) {
        self.ratelimiter.until_ready().await;
    }

    #[cfg(feature = "market_cache")]
    async fn get_from_cache<T>(&self) -> Option<T>
    where
        T: 'static + Send + Sync + Clone,
    {
        self.cache
            .get(&TypeId::of::<T>())
            .await
            .and_then(|item| item.downcast_ref::<T>().cloned())
    }

    #[cfg(feature = "market_cache")]
    async fn insert_into_cache<T>(&self, data: T)
    where
        T: 'static + Send + Sync + Clone,
    {
        self.cache.insert(TypeId::of::<T>(), Arc::new(data)).await;
    }

    /// Fetches the data of a queryable model.
    ///
    /// Note that this function will not cache the data.
    pub async fn fetch<T>(&self) -> Result<T::Data, Error>
    where
        T: Queryable,
        T::Data: Send + Sync + 'static,
    {
        #[cfg(feature = "market_cache")]
        if let Some(data) = self.get_from_cache::<T::Data>().await {
            tracing::debug!(t = type_name::<T>(), "Cache hit");
            return Ok(data);
        }

        ratelimit!(self);

        let fetched = T::query(&self.client).await;

        #[cfg(feature = "market_cache")]
        if let Ok(ref data) = fetched {
            self.insert_into_cache(data.clone()).await;
        }

        fetched
    }

    /// Fetches an item by its slug.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails or if the API returns an error.
    pub async fn fetch_item(&self, item_slug: &str) -> Result<Option<Item>, Error> {
        #[cfg(feature = "market_cache")]
        if let Some(data) = self.get_from_cache::<Item>().await {
            tracing::debug!(t = type_name::<Item>(), "Cache hit");
            return Ok(Some(data));
        }

        ratelimit!(self);

        let response = self
            .client
            .get(format!("https://api.warframe.market/v2/item/{item_slug}"))
            .send()
            .await?
            .json::<ResponseBase<Item>>()
            .await?;

        match response.error {
            Some(error) if error == "app.item.notFound" => Ok(None),
            Some(error) => Err(Error::Api(error)),
            None => {
                let res = response.data.map(Some).ok_or(Error::EmptyErrorAndData);
                #[cfg(feature = "market_cache")]
                if let Ok(Some(ref item)) = res {
                    self.insert_into_cache(item.clone()).await;
                    tracing::debug!(t = type_name::<Item>(), "Cache insertion");
                }

                res
            }
        }
    }
}

macro_rules! ratelimit {
    ($self:expr) => {
        #[cfg(feature = "market_ratelimit")]
        $self.ratelimit().await;
    };
}

use ratelimit;

#[cfg(test)]
mod test {
    use super::Client;
    use crate::market::{
        error::Error,
        queryable::ItemShort,
    };

    #[tokio::test]
    async fn test_cache() -> Result<(), Error> {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();

        let client = Client::new();

        let a = client.fetch::<ItemShort>().await?;
        let b = client.fetch::<ItemShort>().await?;

        assert_eq!(a, b);

        Ok(())
    }
}
