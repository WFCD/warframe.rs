//! # Models
//! All Models can be found here.
//! Some of them are queryable.
//!
//! You can query every model that implements [Queryable](crate::worldstate::models::base::Queryable) [CLient](crate::worldstate::client::Client).
//! # Querying...
//!
//! ### ...through the client
//! To query models through the provided client, see [Client](crate::worldstate::client::Client)
//!
//! ### ...via the [Queryable] trait
//! ```rust
//! use warframe::worldstate::prelude as wf;
//! use warframe::worldstate::prelude::Queryable;
//! #[tokio::main]
//! async fn main() -> Result<(), wf::ApiError> {
//!     let reqwest_client = reqwest::Client::new();
//!
//!     let cetus: wf::Cetus = wf::Cetus::query(&reqwest_client).await?;
//!     let fissures: Vec<wf::Fissure> = wf::Fissure::query(&reqwest_client).await?;
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
// mod global_upgrades;
mod invasion;
pub(crate) mod macros;
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

pub use base::*;

pub use alert::Alert;
pub use arbitration::Arbitration;
pub use archon_hunt::{ArchonHunt, ArchonHuntMission};
pub use cambion_drift::{CambionDrift, CambionDriftState};
pub use cetus::{Cetus, CetusState};
pub use faction::Faction;
pub use fissure::{Fissure, Tier};
pub use flash_sale::FlashSale;
// pub use global_upgrades::GlobalUpgrade;
pub use construction_progress::ConstructionProgress;
pub use daily_deal::DailyDeal;
pub use invasion::{Invasion, InvasionMember};
pub use mission::Mission;
pub use mission_type::MissionType;
pub use news::News;
pub use nightwave::{Nightwave, NightwaveChallenge, NightwaveChallengeType};
pub use orb_vallis::{OrbVallis, OrbVallisState};
pub use reward::{CountedItem, Reward};
pub use reward_type::RewardType;
pub use sortie::{Sortie, SortieMission};
pub use steel_path::{SteelPath, SteelPathShopItem};
pub use syndicate::Syndicate;
pub use syndicate_mission::{SyndicateJob, SyndicateMission};
pub use void_trader::{VoidTrader, VoidTraderInventoryItem};

#[tokio::test]
async fn test_doc_example() -> Result<(), crate::worldstate::prelude::ApiError> {
    use crate::worldstate::prelude as wf;
    use crate::worldstate::prelude::Queryable;

    let client = reqwest::Client::new();

    let _cetus: wf::Cetus = Cetus::query(&client).await?;
    let _fissures: Vec<wf::Fissure> = Fissure::query(&client).await?;

    Ok(())
}
