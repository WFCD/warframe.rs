use crate::worldstate::error::ApiError;

use crate::worldstate::models::base::{Endpoint, Model, RTArray, RTObject};

impl super::Client {
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

#[cfg(not(feature = "multilangual"))]
#[tokio::test]
async fn test_fissure() -> Result<(), ApiError> {
    use crate::worldstate::prelude::*;

    let client = Client::new();

    match client.fetch_arr::<Fissure>().await {
        Ok(_fissures) => Ok(()),
        Err(why) => Err(why),
    }
}
