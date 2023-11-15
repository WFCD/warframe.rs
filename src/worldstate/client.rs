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

#[cfg(test)]
mod test {
    use super::*;
    use crate::worldstate::models::*;

    #[tokio::test]
    async fn test_fetch() -> Result<(), ApiError> {
        let client = Client::new();

        match client.fetch::<Cetus>().await {
            Ok(cetus) => {
                cetus.activation();
                cetus.expiry();
                cetus.start_string();
                cetus.expired();
                cetus.eta();
                Ok(())
            }
            Err(why) => Err(why),
        }
    }

    #[tokio::test]
    async fn test_fetch_arr() -> Result<(), ApiError> {
        let client = Client::new();

        match client.fetch_arr::<Fissure>().await {
            Ok(_fissures) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[tokio::test]
    async fn test_alert() -> Result<(), ApiError> {
        let client = Client::new();

        match client.fetch_arr::<Alert>().await {
            Ok(_alerts) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
