//! Implementation for the market module, used to interact with the warframe.market API
//!
//! This module implements the [market V2 API](https://42bytes.notion.site/WFM-Api-v2-Documentation-5d987e4aa2f74b55a80db1a09932459d).
#[cfg(feature = "market_cache")]
pub(crate) mod cache;
mod client;
pub mod error;
pub(crate) mod models;

pub use client::Client;
pub use error::{
    Error,
    Result,
};
pub use models::{
    Queryable,
    ResponseBase,
};

/// Re-export of all the models that are queryable
pub mod queryable {
    pub use super::models::{
        item::Item,
        item_short::ItemShort,
        set_items::SetItems,
        versions::Versions,
    };
}

pub const BASE_URL: &str = "https://api.warframe.market/v2";
