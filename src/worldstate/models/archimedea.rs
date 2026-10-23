use warframe_macros::model;

use super::faction::Faction;
use crate::worldstate::MissionType;

/// The type of an Archimedea challenge rotation
#[derive(
    Debug, PartialEq, PartialOrd, Clone, Eq, Hash, derive_more::Display, serde::Deserialize,
)]
pub enum ArchimedeaType {
    /// Circuit Lab
    #[serde(rename = "C T_ L A B")]
    Lab,
    /// Circuit Hex
    #[serde(rename = "C T_ H E X")]
    Hex,
    /// An unrecognized Archimedea type
    #[serde(untagged)]
    Unknown(String),
}

/// A modifier applied to an Archimedea mission
#[model]
pub struct ArchimedeaModifier {
    /// The internal key for this modifier
    pub key: String,
    /// The display name of this modifier
    pub name: String,
    /// A description of the modifier's effect
    pub description: String,
}

/// A risk modifier applied to an Archimedea mission
#[model]
pub struct ArchimedeaRisk {
    /// The internal key for this risk
    pub key: String,
    /// The display name of this risk
    pub name: String,
    /// A description of the risk's effect
    pub description: String,
    /// Whether this is a hard-mode risk
    pub is_hard: bool,
}

/// A mission within an Archimedea rotation
#[model]
pub struct ArchimedeaMission {
    /// The i18n faction you are up against
    pub faction: String,
    /// The faction you are up against
    pub faction_key: Faction,
    /// The i18n mission type
    pub mission_type: String,
    /// The type of the mission
    pub mission_type_key: MissionType,
    /// The deviation modifier for this mission
    pub deviation: ArchimedeaModifier,
    /// The risk modifiers applied to this mission
    pub risks: Vec<ArchimedeaRisk>,
}

/// An Archimedea (Deep Archimedea) challenge rotation
#[model(endpoint = "/archimedeas", return_style = Array, timed)]
pub struct Archimedea {
    /// Unique identifier for this rotation
    pub id: String,
    /// The i18n Archimedea type
    #[serde(rename = "type")]
    pub archimedea_type: String,
    /// The type of this Archimedea rotation
    pub type_key: ArchimedeaType,
    /// The missions in this rotation
    pub missions: Vec<ArchimedeaMission>,
    /// Personal modifiers for this rotation
    pub personal_modifiers: Vec<ArchimedeaModifier>,
}

#[cfg(test)]
mod test_archimedea {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Archimedea;
    use crate::worldstate::Queryable;

    type R = <Archimedea as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/archimedea.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
