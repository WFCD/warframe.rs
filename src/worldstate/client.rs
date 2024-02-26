use log::debug;

use super::error::ApiError;
use super::models::base::{Endpoint, Model, RTArray, RTObject, TimedEvent};
use std::future::Future;

pub trait FnHelper<'a, T: Sized> {
    fn call(&self, before: &'a T, after: &'a T) -> impl Future + 'a;
}

impl<'a, T, Fut, Func> FnHelper<'a, T> for Func
where
    T: Sized + 'a,
    Fut: Future + 'a,
    Func: Fn(&'a T, &'a T) -> Fut,
{
    fn call(&self, before: &'a T, after: &'a T) -> impl Future + 'a {
        self(before, after)
    }
}

#[derive(Default)]
pub struct Client {
    session: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn call_on_update<T, Func>(&self, callback: Func) -> Result<(), ApiError>
    where
        T: Model + Endpoint + RTObject + TimedEvent,
        for<'any> Func: FnHelper<'any, T>,
    {
        let mut item = self.fetch::<T>().await?;

        loop {
            if item.expiry() <= chrono::offset::Utc::now() {
                debug!(
                    "{} :: Looking for state change from the API",
                    std::any::type_name::<T>()
                );
                tokio::time::sleep(std::time::Duration::from_secs(30)).await;

                let new_item = self.fetch::<T>().await?;

                if item.expiry() >= new_item.expiry() {
                    continue;
                }
                callback.call(&item, &new_item).await;
                item = new_item;
            }

            let time_to_sleep = item.expiry() - chrono::offset::Utc::now();

            debug!(
                "{} :: Sleeping {} seconds",
                std::any::type_name::<T>(),
                time_to_sleep.num_seconds()
            );
            tokio::time::sleep(time_to_sleep.to_std().unwrap()).await;
        }
    }

    pub async fn fetch<T>(&self) -> Result<T, ApiError>
    where
        T: Model + Endpoint + RTObject,
    {
        let response = self.session.get(T::endpoint_en()).send().await.unwrap();
        match response.status().as_u16() {
            200 => Ok(response.json::<T>().await.unwrap()), // unwrap should be safe - the API only responds with a JSON
            _code => Err(ApiError::from(response).await),
        }
    }

    pub async fn fetch_arr<T>(&self) -> Result<Vec<T>, ApiError>
    where
        T: Model + Endpoint + RTArray,
    {
        let response = self.session.get(T::endpoint_en()).send().await.unwrap();
        match response.status().as_u16() {
            200 => Ok(response.json::<Vec<T>>().await.unwrap()), // unwrap should be safe - the API only responds with a JSON
            _code => Err(ApiError::from(response).await),
        }
    }
}

#[cfg(feature = "multilangual")]
impl Client {
    pub async fn fetch_using_lang<T>(
        &self,
        language: super::language::Language,
    ) -> Result<T, ApiError>
    where
        T: Model + Endpoint + RTObject,
    {
        let response = self
            .session
            .get(T::endpoint(language))
            .send()
            .await
            .unwrap();
        match response.status().as_u16() {
            200 => Ok(response.json::<T>().await.unwrap()), // unwrap should be safe - the API only responds with a JSON
            _code => Err(ApiError::from(response).await),
        }
    }

    pub async fn fetch_arr_using_lang<T>(
        &self,
        language: super::language::Language,
    ) -> Result<Vec<T>, ApiError>
    where
        T: Model + Endpoint + RTArray,
    {
        let response = self
            .session
            .get(T::endpoint(language))
            .send()
            .await
            .unwrap();

        match response.status().as_u16() {
            200 => Ok(response.json::<Vec<T>>().await.unwrap()), // unwrap should be safe - the API only responds with a JSON
            _code => Err(ApiError::from(response).await),
        }
    }
}
