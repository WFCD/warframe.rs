//! # Models
//! All Models can be found here.
//! Some of them are queryable.
//!
//! You can query every model that implements
//! [Queryable](crate::worldstate::models::base::Queryable)
//! [`Client`](crate::worldstate::client::Client). # Querying...
//!
//! ### ...through the client
//! To query models through the provided client, see [Client](crate::worldstate::client::Client)
//!
//! ### ...via the [Queryable] trait
//! ```rust
//! use warframe::worldstate::{
//!     Client,
//!     Error,
//!     Queryable,
//!     queryable::{
//!         Cetus,
//!         Fissure,
//!     },
//! };
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let reqwest_client = reqwest::Client::new();
//!
//!     let cetus: Cetus = Cetus::query(&reqwest_client).await?;
//!     let fissures: Vec<Fissure> = Fissure::query(&reqwest_client).await?;
//!
//!     Ok(())
//! }
//! ```

pub mod alert;
pub mod arbitration;
pub mod archon_hunt;
pub mod base;
pub mod cambion_drift;
pub mod cetus;
pub mod construction_progress;
pub mod daily_deal;
pub mod event;
pub mod faction;
pub mod fissure;
pub mod flash_sale;
pub mod items;
// mod global_upgrades;
pub mod damage_type;
pub mod deep_archimedia;
pub mod global_upgrades;
pub mod invasion;
pub mod mission;
pub mod mission_type;
pub mod news;
pub mod nightwave;
pub mod orb_vallis;
pub mod reward;
pub mod reward_type;
pub mod sortie;
pub mod steel_path;
pub mod syndicate;
pub mod syndicate_mission;
pub mod void_trader;

use items::Item;

use super::{
    client::Client,
    error::Error,
    language::Language,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize)]
pub struct ItemStringWrapper(String);

impl ItemStringWrapper {
    /// Queries an item using the provided client.
    ///
    /// This is a convenience function.
    ///
    /// # Arguments
    ///
    /// * `client` - The client used to query the item.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<Item>` if the query is successful, or an `Error` if it
    /// fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the client fails to query the item.
    pub async fn query(&self, client: Client) -> Result<Option<Item>, Error> {
        client.query_item(&self.0).await
    }

    /// Queries an item using the provided client with the provided localization
    ///
    /// This is a convenience function.
    ///
    /// # Arguments
    ///
    /// * `client` - The client used to query the item.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<Item>` if the query is successful, or an `Error` if it
    /// fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the client fails to query the item.
    pub async fn query_using_lang(
        &self,
        client: Client,
        language: Language,
    ) -> Result<Option<Item>, Error> {
        client.query_item_using_lang(&self.0, language).await
    }

    #[must_use]
    pub fn inner(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for ItemStringWrapper {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
