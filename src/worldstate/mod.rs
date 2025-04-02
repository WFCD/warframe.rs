//! # The worldstate module
//!
//! Get information about various parts of the game.
//!
//! ## Quickstart
//! ```rust,no_run
//! use warframe::worldstate::{
//!     Client,
//!     Error,
//!     queryable::{
//!         Cetus,
//!         Fissure,
//!     },
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let client = Client::new();
//!
//!     let cetus: Cetus = client.fetch::<Cetus>().await?;
//!     let fissures: Vec<Fissure> = client.fetch::<Fissure>().await?;
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
mod models;
pub mod utils;

pub mod language;

#[cfg(test)]
pub(crate) mod fixtures;

/// A module that re-exports every type that is queryable
pub mod queryable {
    pub use super::models::{
        alert::Alert,
        arbitration::Arbitration,
        archon_hunt::ArchonHunt,
        cambion_drift::CambionDrift,
        cetus::Cetus,
        construction_progress::ConstructionProgress,
        daily_deal::DailyDeal,
        deep_archimedia::DeepArchimedia,
        event::Event,
        fissure::Fissure,
        flash_sale::FlashSale,
        global_upgrades::GlobalUpgrade,
        invasion::Invasion,
        news::News,
        nightwave::Nightwave,
        orb_vallis::OrbVallis,
        sortie::Sortie,
        steel_path::SteelPath,
        syndicate::Syndicate,
        void_trader::VoidTrader,
    };
}

pub use client::Client;
pub use error::{
    ApiErrorResponse,
    Error,
};
pub use language::Language;
pub use models::{
    archon_hunt::ArchonHuntMission,
    base::{
        Endpoint,
        Opposite,
        Queryable,
        TimedEvent,
    },
    cambion_drift::CambionDriftState,
    cetus::CetusState,
    damage_type::{
        CombinedElementalDamage,
        DamageType,
        ElementalDamage,
        PhysicalDamage,
    },
    deep_archimedia::{
        DeepArchimedeaModifier,
        DeepArchimediaMission,
    },
    faction::Faction,
    fissure::Tier,
    invasion::InvasionMember,
    items,
    mission::Mission,
    mission_type::MissionType,
    nightwave::{
        NightwaveChallenge,
        NightwaveChallengeType,
    },
    orb_vallis::OrbVallisState,
    reward::Reward,
    reward_type::RewardType,
    sortie::SortieMission,
    steel_path::SteelPathShopItem,
    syndicate_mission::{
        SyndicateJob,
        SyndicateMission,
    },
    void_trader::VoidTraderInventoryItem,
};
pub use utils::Change;
/// This is a re-export of the `model` macro in case you want to use it in your own code.
/// To implement a, for example, missing model.
pub use warframe_macros::model;
