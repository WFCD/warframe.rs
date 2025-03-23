//! Implementation for the market module, used to interact with the warframe.market API
//!
//! This module implements the [market V2 API](https://42bytes.notion.site/WFM-Api-v2-Documentation-5d987e4aa2f74b55a80db1a09932459d).
mod client;
pub mod error;
pub mod models;

pub use client::Client;
pub use error::{
    Error,
    Result,
};
pub use models::base::{
    Endpoint,
    Queryable,
    ResponseBase,
};

pub mod queryable {
    pub use super::models::item_short::ItemShort;
}

pub const BASE_URL: &str = "https://api.warframe.market/v2";
