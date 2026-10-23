use warframe_macros::model;

use super::{
    faction::Faction,
    mission_type::MissionType,
};

/// A Mission corresponding to a Sortie
#[model]
pub struct SortieMission {
    /// The i18n Mission type of this mission
    pub mission_type: String,

    /// The type of the mission
    pub mission_type_key: MissionType,

    /// The Modifier of this mission (e.g. Augmented Enemy Armor)
    pub modifier: String,

    /// The description of the modifier of this mission
    pub modifier_description: String,

    /// The i18n of the node
    pub node: String,

    /// The name of the node
    pub node_key: String,
}

/// Data about the missions for the current sortie
#[model(endpoint = "/sortie", return_style = Object, timed)]
pub struct Sortie {
    /// Unique identifier for this object/event/thing
    pub id: String,

    /// The name of the boss
    pub boss: String,

    /// The i18n of the faction you are up against
    pub faction: String,

    /// The key of the faction
    pub faction_key: Option<Faction>,

    /// The reward pool for this sortie
    pub reward_pool: Option<String>,

    /// The 3 missions corresponding to this sortie
    #[serde(rename = "variants")]
    pub missions: [SortieMission; 3],
}

#[cfg(test)]
mod test_sortie {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Sortie;
    use crate::worldstate::Queryable;

    type R = <Sortie as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/sortie.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
