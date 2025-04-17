pub mod activity;
pub mod i18n;
pub mod item;
pub mod item_short;
pub mod lich_ephemera;
pub mod lich_quirk;
pub mod lich_weapon;
pub mod location;
pub mod mission;
pub mod npc;
pub mod order;
pub mod order_with_user;
pub mod riven;
pub mod riven_attribute;
pub mod riven_group;
pub mod riven_type;
pub mod set_items;
pub mod sister_ephemera;
pub mod sister_quirk;
pub mod sister_weapon;
pub mod top_orders;
pub mod top_orders_query_params;
pub mod user_short;
pub mod versions;

use std::fmt::Debug;

use i18n::Language;
use serde::{
    Deserialize,
    de::DeserializeOwned,
};

use super::BASE_URL;
use crate::market::error::Error;

#[derive(Debug, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBase<T> {
    pub api_version: String,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub trait Queryable: Debug {
    const ENDPOINT: &str;
    type Data: DeserializeOwned + Clone + Debug + Send + Sync + 'static;

    #[must_use]
    fn query(
        client: &reqwest::Client,
        language: Language,
    ) -> impl Future<Output = Result<Self::Data, Error>> + Send {
        async move {
            let response = client
                .get(format!("{}{}", BASE_URL, Self::ENDPOINT))
                .header("Language", language.to_string())
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

macro_rules! impl_queryable {
    ($name:ident,Array, $endpoint:literal) => {
        impl crate::market::Queryable for $name {
            const ENDPOINT: &str = $endpoint;
            type Data = Vec<Self>;
        }
    };
    ($name:ident,Object, $endpoint:literal) => {
        impl crate::market::Queryable for $name {
            const ENDPOINT: &str = $endpoint;
            type Data = Self;
        }
    };
}

pub(crate) use impl_queryable;
