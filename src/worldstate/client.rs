use super::error::ApiError;
use super::models::base::{Endpoint, Model};

static BASE_URL: &str = "https://api.warframestat.us/pc";

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
        let endpoint = format!("{}{}", BASE_URL, T::get_endpoint());
        let response = self.session.get(endpoint).send().await.unwrap();
        match response.status().as_u16() {
            200 => Ok(response.json::<T>().await.unwrap()),
            _ => Err(response.json::<ApiError>().await.unwrap()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::worldstate::models::cetus::Cetus;

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
