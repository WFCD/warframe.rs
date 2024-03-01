use log::debug;

use crate::worldstate::utils::CrossDiff;

use super::error::ApiError;
use super::listener::{ArrayListenerCallback, ObjectListenerCallback};
use super::models::base::{Endpoint, Model, RTArray, RTObject, TimedEvent};

#[derive(Default)]
pub struct Client {
    session: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn call_on_update<T, Callback>(&self, callback: Callback) -> Result<(), ApiError>
    where
        T: Model + Endpoint + RTObject + TimedEvent,
        for<'any> Callback: ObjectListenerCallback<'any, T>,
    {
        let mut item = self.fetch::<T>().await?;

        loop {
            if item.expiry() <= chrono::offset::Utc::now() {
                debug!(
                    "{} :: Fetching new possible state",
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

    pub async fn call_on_any_update<T, Callback>(&self, callback: Callback) -> Result<(), ApiError>
    where
        T: Model + Endpoint + RTArray + TimedEvent + PartialEq,
        for<'any> Callback: ArrayListenerCallback<'any, T>,
    {
        let mut items = self.fetch_arr::<T>().await?;

        loop {
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;

            debug!(
                "{} :: Fetching new possible state",
                std::any::type_name::<T>()
            );
            let new_items = self.fetch_arr::<T>().await?;

            let diff = CrossDiff::new(&items, &new_items);

            let removed_items = diff.removed();
            let added_items = diff.added();

            if !removed_items.is_empty() && !added_items.is_empty() {
                debug!(
                    "{} :: Found changes, proceeding to call callback with every change",
                    std::any::type_name::<T>()
                );
                for item in removed_items.into_iter().chain(added_items) {
                    // call callback fn
                    callback.call(item).await;
                }
                items = new_items;
            }
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
