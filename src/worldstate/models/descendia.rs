use warframe_macros::model;

/// The mission type key for a Descendia challenge
#[derive(
    Debug, PartialEq, PartialOrd, Clone, Eq, Hash, derive_more::Display, serde::Deserialize,
)]
pub enum DescentMissionType {
    /// Assassination
    Assassination,
    /// Disruption
    Disruption,
    /// Extermination
    Extermination,
    /// Survival
    Survival,
    /// Circuit Hex
    #[serde(rename = "C T_ H E X")]
    CircuitHex,
    /// Circuit Lab
    #[serde(rename = "C T_ L A B")]
    CircuitLab,
    /// Alchemy
    #[serde(rename = "DT_ALCHEMY")]
    Alchemy,
    /// Break Targets
    #[serde(rename = "DT_BREAK_TARGETS")]
    BreakTargets,
    /// Capture
    #[serde(rename = "DT_CAPTURE")]
    Capture,
    /// Collection
    #[serde(rename = "DT_COLLECTION")]
    Collection,
    /// Defense
    #[serde(rename = "DT_DEFENSE")]
    Defense,
    /// Excavation
    #[serde(rename = "DT_EXCAVATION")]
    Excavation,
    /// Exterminate
    #[serde(rename = "DT_EXTERMINATE")]
    Exterminate,
    /// Infested Salvage
    #[serde(rename = "DT_INFESTED_SALVAGE")]
    InfestedSalvage,
    /// Interception
    #[serde(rename = "DT_INTERCEPTION")]
    Interception,
    /// Loot Creatures
    #[serde(rename = "DT_LOOT_CREATURES")]
    LootCreatures,
    /// Mimics
    #[serde(rename = "DT_MIMICS")]
    Mimics,
    /// Netracells
    #[serde(rename = "DT_NETRACELLS")]
    Netracells,
    /// Pressure Gauge
    #[serde(rename = "DT_PRESURE_GAUGE")]
    PressureGauge,
    /// Protoframe
    #[serde(rename = "DT_PROTOFRAME")]
    Protoframe,
    /// Race
    #[serde(rename = "DT_RACE")]
    Race,
    /// Sabotage Defense
    #[serde(rename = "DT_SABOTAGE_DEFENSE")]
    SabotageDefense,
    /// Sabotage Hive
    #[serde(rename = "DT_SABOTAGE_HIVE")]
    SabotageHive,
    /// Shrine Defense
    #[serde(rename = "DT_SHRINE_DEFENSE")]
    ShrineDefense,
    /// Unique
    #[serde(rename = "DT_UNIQUE")]
    Unique,
    /// An unrecognized mission type
    #[serde(untagged)]
    Unknown(String),
}

/// The challenge type key for a Descendia challenge
#[derive(
    Debug, PartialEq, PartialOrd, Clone, Eq, Hash, derive_more::Display, serde::Deserialize,
)]
pub enum DescentChallengeType {
    /// Arbitration Drones
    ArbitrationDrones,
    /// Basic Loot Creatures
    BasicLootCreatures,
    /// Basic Mimics
    BasicMimics,
    /// Basic Race
    BasicRace,
    /// Devil
    Devil,
    /// Escapist
    Escapist,
    /// Fiery Trail Rollers
    FieryTrailRollers,
    /// Fire Chain
    FireChain,
    /// Freeze In Shoot
    FreezeInShoot,
    /// Giant Realm
    GiantRealm,
    /// Glass Maker
    GlassMaker,
    /// Hard Shell
    HardShell,
    /// Harrow
    Harrow,
    /// Head Shots Only
    HeadShotsOnly,
    /// Heavy Weapons Only
    HeavyWeaponsOnly,
    /// Jump Smash
    JumpSmash,
    /// Manics
    Manics,
    /// Netracell Darkness
    #[serde(rename = "NC_Darkness")]
    NetracellDarkness,
    /// Netracell Mine Field
    #[serde(rename = "NC_MineField")]
    NetracellMineField,
    /// Netracell Security Spin
    #[serde(rename = "NC_SecuritySpin")]
    NetracellSecuritySpin,
    /// Netracell Spike Ceiling
    #[serde(rename = "NC_SpikeCeiling")]
    NetracellSpikeCeiling,
    /// Poison Gas
    PoisonGas,
    /// Power House
    PowerHouse,
    /// Race Horse
    RaceHorse,
    /// Rockets Only
    RocketsOnly,
    /// Security Spin
    SecuritySpin,
    /// Slip and Slide
    SlipAndSlide,
    /// Spicy Knife
    SpicyKnife,
    /// Spike Ceiling
    SpikeCeiling,
    /// Toxic Fire
    ToxicFire,
    /// Void Aberration
    VoidAberration,
    /// Wisp
    Wisp,
    /// An unrecognized challenge type
    #[serde(untagged)]
    Unknown(String),
}

/// A named item reference used in Descendia challenges
#[model]
pub struct DescendiaNamedItem {
    /// The internal unique name of the item
    pub unique_name: String,
    /// The display name of the item
    pub name: String,
}

/// A challenge within a Descendia rotation
#[model]
pub struct DescendiaChallenge {
    /// The position of this challenge in the rotation
    pub index: i32,
    /// The i18n mission type
    #[serde(rename = "type")]
    pub challenge_type: String,
    /// The mission type key
    pub type_key: DescentMissionType,
    /// The i18n challenge name
    pub challenge: String,
    /// The challenge type key
    pub challenge_key: DescentChallengeType,
    /// The i18n level/node name
    pub level: String,
    /// The internal unique name of the level
    pub level_unique_name: String,
    /// The Warframe specializations available for this challenge
    pub specs: Vec<DescendiaNamedItem>,
    /// The auras available for this challenge
    pub auras: Vec<DescendiaNamedItem>,
}

/// The current Descendia (Circuit of Heroes) rotation
#[model(endpoint = "/descendia", return_style = Object, timed)]
pub struct Descendia {
    /// Unique identifier for this rotation
    pub id: String,
    /// The seed used to generate this rotation
    pub seed: u64,
    /// The challenges in this rotation
    pub challenges: Vec<DescendiaChallenge>,
}

#[cfg(test)]
mod test_descendia {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Descendia;
    use crate::worldstate::Queryable;

    type R = <Descendia as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/descendia.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
