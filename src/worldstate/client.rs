use super::error::ApiError;
use super::models::base::{Endpoint, Model, RTArray, RTObject};

#[derive(Default)]
pub struct Client {
    session: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn fetch<T>(&self) -> Result<T, ApiError>
    where
        T: Model + Endpoint + RTObject,
    {
        let response = self.session.get(T::endpoint()).send().await.unwrap();
        match response.status().as_u16() {
            200 => Ok(response.json::<T>().await.unwrap()), // unwrap should be safe - the API only responds with a JSON
            _code => Err(ApiError::from(response).await),
        }
    }

    pub async fn fetch_arr<T>(&self) -> Result<Vec<T>, ApiError>
    where
        T: Model + Endpoint + RTArray,
    {
        let response = self.session.get(T::endpoint()).send().await.unwrap();
        match response.status().as_u16() {
            200 => Ok(response.json::<Vec<T>>().await.unwrap()), // unwrap should be safe - the API only responds with a JSON
            _code => Err(ApiError::from(response).await),
        }
    }
}
