#![allow(clippy::doc_markdown)]
use warframe_macros::model;

/// A Mission Type in Warframe
#[model]
pub enum MissionType {
    /// AncientRetribution
    #[serde(rename = "Ancient Retribution")]
    AncientRetribution,
    /// Arena
    Arena,
    /// Assassination
    Assassination,
    /// Assault
    Assault,
    /// Capture
    Capture,
    /// Conclave
    Conclave,
    /// DarkSectorDefection
    #[serde(rename = "Dark Sector Defection")]
    DarkSectorDefection,
    /// DarkSectorDefense
    #[serde(rename = "Dark Sector Defense")]
    DarkSectorDefense,
    /// DarkSectorDisruption
    #[serde(rename = "Dark Sector Disruption")]
    DarkSectorDisruption,
    /// DarkSectorExcavation
    #[serde(rename = "Dark Sector Excavation")]
    DarkSectorExcavation,
    /// DarkSectorSabotage
    #[serde(rename = "Dark Sector Sabotage")]
    DarkSectorSabotage,
    /// DarkSectorSurvival
    #[serde(rename = "Dark Sector Survival")]
    DarkSectorSurvival,
    /// Defense
    Defense,
    /// Disruption
    Disruption,
    /// Excavation
    Excavation,
    /// ExterminationArchwing
    #[serde(rename = "Extermination (Archwing)")]
    ExterminationArchwing,
    /// Extermination
    Extermination,
    /// FreeRoam
    #[serde(rename = "Free Roam")]
    FreeRoam,
    /// Hijack
    Hijack,
    /// Hive
    Hive,
    /// HiveSabotage
    #[serde(rename = "Hive Sabotage")]
    HiveSabotage,
    /// Interception
    Interception,
    /// InterceptionArchwing
    #[serde(rename = "Interception (Archwing)")]
    InterceptionArchwing,
    /// MobileDefense
    #[serde(rename = "Mobile Defense")]
    MobileDefense,
    /// MobileDefenseArchwing
    #[serde(rename = "Mobile Defense (Archwing)")]
    MobileDefenseArchwing,
    /// OrokinSabotage
    #[serde(rename = "Orokin Sabotage")]
    OrokinSabotage,
    /// Orphix
    Orphix,
    /// PursuitArchwing
    #[serde(rename = "Pursuit (Archwing)")]
    PursuitArchwing,
    /// Relay
    Relay,
    /// Rescue
    Rescue,
    /// RushArchwing
    #[serde(rename = "Rush (Archwing)")]
    RushArchwing,
    /// Sabotage
    Sabotage,
    /// SabotageArchwing
    #[serde(rename = "Sabotage (Archwing)")]
    SabotageArchwing,
    /// Skirmish
    Skirmish,
    /// Spy
    Spy,
    /// Survival
    Survival,
    /// Volatile
    Volatile,
    /// Alchemy
    Alchemy,
    /// Corruption
    Corruption,
    /// VoidCascade
    #[serde(rename = "Void Cascade")]
    VoidCascade,
    /// Defection
    Defection,
    /// Unknown
    Unknown,
}
