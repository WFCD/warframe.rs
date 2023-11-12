use super::error::ApiError;
use super::models::base::{Endpoint, Model};

#[derive(Default)]
pub struct Client {
    session: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn fetch<'a, T>(&self) -> Result<T, ApiError>
    where
        T: Model + Endpoint,
    {
        let response = self.session.get(T::endpoint()).send().await.unwrap();
        match response.status().as_u16() {
            200 => Ok(response.json::<T>().await.unwrap()), // unwrap should be safe - the API only responds with a JSON
            _ => Err(response.json::<ApiError>().await.unwrap()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::worldstate::models::Cetus;

    #[tokio::test]
    async fn test_fetch() -> Result<(), ()> {
        let client = Client::new();

        match client.fetch::<Cetus>().await {
            Ok(cetus) => {
                println!("{cetus:?}");
                Ok(())
            }
            Err(why) => {
                println!("{why:?}");
                Err(())
            }
        }
    }
}
