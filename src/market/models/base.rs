use serde::{
    Deserialize,
    de::DeserializeOwned,
};

use crate::market::error::Error;

#[derive(Debug, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBase<T> {
    pub api_version: String,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub trait Endpoint {
    fn endpoint() -> &'static str;
}

pub trait Queryable: Endpoint + DeserializeOwned {
    type Data: DeserializeOwned + Clone;

    #[must_use]
    fn query(client: &reqwest::Client) -> impl Future<Output = Result<Self::Data, Error>> + Send {
        async {
            let response = client
                .get(Self::endpoint())
                .send()
                .await?
                .json::<ResponseBase<Self::Data>>()
                .await?;

            if let Some(error) = response.error {
                return Err(Error::Api(error));
            }

            response.data.ok_or(Error::EmptyErrorAndData)
        }
    }
}

macro_rules! impl_endpoint {
    ($name:ident, $endpoint:expr) => {
        impl crate::market::Endpoint for $name {
            fn endpoint() -> &'static str {
                concat!("https://api.warframe.market/v2", $endpoint)
            }
        }
    };
}

pub(crate) use impl_endpoint;

macro_rules! impl_queryable {
    ($name:ident,Array) => {
        impl crate::market::Queryable for $name {
            type Data = Vec<Self>;
        }
    };
    ($name:ident,Object) => {
        impl crate::market::Queryable for $name {
            type Data = Self;
        }
    };
}

pub(crate) use impl_queryable;
