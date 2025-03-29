//! Implementation for the market module, used to interact with the warframe.market API
//!
//! This module implements the [market V2 API](https://42bytes.notion.site/WFM-Api-v2-Documentation-5d987e4aa2f74b55a80db1a09932459d).
#[cfg(feature = "market_cache")]
pub(crate) mod cache;
mod client;
pub mod error;
pub(crate) mod models;

pub use client::Client;
use derive_more::{
    AsRef,
    Display,
    Into,
};
pub use error::{
    Error,
    Result,
};
use heck::ToSnakeCase;
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

/// This is a utility newtype struct to help convert user inputs (such as `Acceltra Prime Set`) into
/// slugs (e.g. `acceltra_prime_set`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Into, AsRef)]
#[as_ref(str)]
pub struct Slug(String);

impl Slug {
    #[must_use]
    pub fn new(input: &str) -> Self {
        Self(input.to_snake_case())
    }
}

impl From<String> for Slug {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}

impl From<&str> for Slug {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl PartialEq<String> for Slug {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl PartialEq<str> for Slug {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for Slug {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

#[cfg(test)]
mod test {
    use super::Slug;

    #[test]
    fn test_slug() {
        assert_eq!(Slug::new("Acceltra Prime Set"), "acceltra_prime_set");
        assert_eq!(Slug::new("Mirage Prime Set"), "mirage_prime_set");
        assert_eq!(Slug::new("Riot-848 Barrel"), "riot_848_barrel");
    }
}
