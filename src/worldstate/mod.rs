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
//!     let client = Client::default();
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
mod listener;
mod listener_nested;

/// A module that re-exports every type that is queryable
pub mod queryable {
    pub use super::models::{
        alert::Alert,
        arbitration::Arbitration,
        archimedea::Archimedea,
        archon_hunt::ArchonHunt,
        calendar::Calendar,
        cambion_drift::CambionDrift,
        cetus::Cetus,
        clan_weekly_initiative::ClanWeeklyInitiative,
        conclave_challenge::ConclaveChallenge,
        construction_progress::ConstructionProgress,
        daily_deal::DailyDeal,
        descendia::Descendia,
        duviri_cycle::DuviriCycle,
        earth_cycle::EarthCycle,
        event::Event,
        faceoff_bonus::FaceoffBonus,
        fissure::Fissure,
        flash_sale::FlashSale,
        global_upgrades::GlobalUpgrade,
        invasion::Invasion,
        kinepage::Kinepage,
        news::News,
        nightwave::Nightwave,
        orb_vallis::OrbVallis,
        sentient_outpost::SentientOutpost,
        simaris::Simaris,
        sortie::Sortie,
        steel_path::SteelPath,
        syndicate::Syndicate,
        vault_trader::VaultTrader,
        void_trader::VoidTrader,
        zariman_cycle::ZarimanCycle,
    };
}

pub use client::Client;
pub use error::{
    ApiErrorResponse,
    Error,
};
pub use language::Language;
pub use models::{
    archimedea::{
        ArchimedeaMission,
        ArchimedeaModifier,
        ArchimedeaRisk,
        ArchimedeaType,
    },
    archon_hunt::ArchonHuntMission,
    base::{
        Endpoint,
        Opposite,
        Queryable,
        TimedEvent,
    },
    calendar::{
        CalendarChallenge,
        CalendarDay,
        CalendarEvent,
        CalendarUpgrade,
    },
    cambion_drift::CambionDriftState,
    cetus::CetusState,
    clan_weekly_initiative::ClanWeeklyReward,
    conclave_challenge::ConclaveChallengeCategory,
    damage_type::{
        CombinedElementalDamage,
        DamageType,
        ElementalDamage,
        PhysicalDamage,
    },
    descendia::{
        DescendiaChallenge,
        DescendiaNamedItem,
        DescentChallengeType,
        DescentMissionType,
    },
    duviri_cycle::{
        DuviriChoice,
        DuviriChoiceCategory,
    },
    faction::{
        EnemyFaction,
        Faction,
        SyndicateFaction,
        TennoFaction,
    },
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
    sentient_outpost::SentientOutpostMission,
    sortie::SortieMission,
    steel_path::{
        SteelPathIncursion,
        SteelPathShopItem,
    },
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
