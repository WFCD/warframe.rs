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
//!     client::Client,
//!     error::Error,
//!     models::{
//!         Cetus,
//!         Fissure,
//!         Queryable,
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

mod alert;
mod arbitration;
mod archon_hunt;
pub mod base;
mod cambion_drift;
mod cetus;
mod construction_progress;
mod daily_deal;
mod event;
mod faction;
mod fissure;
mod flash_sale;
pub mod items;
// mod global_upgrades;
mod damage_type;
pub mod global_upgrades;
mod invasion;
mod mission;
mod mission_type;
mod news;
mod nightwave;
mod orb_vallis;
mod reward;
mod reward_type;
mod sortie;
mod steel_path;
mod syndicate;
mod syndicate_mission;
mod void_trader;

pub use alert::Alert;
pub use arbitration::Arbitration;
pub use archon_hunt::{
    ArchonHunt,
    ArchonHuntMission,
};
pub use base::*;
pub use cambion_drift::{
    CambionDrift,
    CambionDriftState,
};
pub use cetus::{
    Cetus,
    CetusState,
};
// pub use global_upgrades::GlobalUpgrade;
pub use construction_progress::ConstructionProgress;
pub use daily_deal::DailyDeal;
pub use damage_type::{
    CombinedElementalDamage,
    DamageType,
    ElementalDamage,
    PhysicalDamage,
};
pub use faction::Faction;
pub use fissure::{
    Fissure,
    Tier,
};
pub use flash_sale::FlashSale;
pub use invasion::{
    Invasion,
    InvasionMember,
};
use items::Item;
pub use mission::Mission;
pub use mission_type::MissionType;
pub use news::News;
pub use nightwave::{
    Nightwave,
    NightwaveChallenge,
    NightwaveChallengeType,
};
pub use orb_vallis::{
    OrbVallis,
    OrbVallisState,
};
pub use reward::{
    CountedItem,
    Reward,
};
pub use reward_type::RewardType;
pub use sortie::{
    Sortie,
    SortieMission,
};
pub use steel_path::{
    SteelPath,
    SteelPathShopItem,
};
pub use syndicate::Syndicate;
pub use syndicate_mission::{
    SyndicateJob,
    SyndicateMission,
};
pub use void_trader::{
    VoidTrader,
    VoidTraderInventoryItem,
};

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

#[tokio::test]
async fn test_doc_example() -> Result<(), crate::worldstate::error::Error> {
    let client = reqwest::Client::new();

    let _cetus: Cetus = Cetus::query(&client).await?;
    let _fissures: Vec<Fissure> = Fissure::query(&client).await?;

    Ok(())
}
