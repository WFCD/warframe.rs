use warframe_macros::model;

use super::{
    faction::Faction,
    mission_type::MissionType,
};

/// An archon hunt mission
#[model]
pub struct ArchonHuntMission {
    /// The i18n of the node
    pub node: String,

    /// The name of the node
    pub node_key: String,

    /// The i18n type of the mission
    pub r#type: String,

    /// The type of the mission
    pub type_key: MissionType,

    /// Whether the mission requires an archwing
    pub archwing_required: bool,

    /// Whether the mission is a sharkwing mission
    pub is_sharkwing: bool,

    /// Any additional spawners
    pub advanced_spawners: Vec<String>,

    /// Items required to enter the mission
    pub required_items: Vec<String>,

    /// Affectors of this mission
    pub level_auras: Vec<String>,
}

/// An alert in Warframe
#[model(endpoint = "/archonHunt", return_style = Object, timed)]
pub struct ArchonHunt {
    /// ID of this event
    pub id: String,

    /// The Archon you are up against
    pub boss: String,

    /// The Reward Pool of this event
    pub reward_pool: String,

    /// The i18n of the faction you are up against
    pub faction: String,

    /// The faction you are up against
    pub faction_key: Option<Faction>,

    /// The missions associated with this week's Archon Hunt
    pub missions: [ArchonHuntMission; 3],
}

#[cfg(test)]
mod test_archonhunt {
    use rstest::rstest;
    use serde_json::from_str;

    use super::ArchonHunt;
    use crate::worldstate::{
        Queryable,
        fixtures::archon_hunt::{
            archon_hunt,
            archon_hunt_en,
        },
    };

    type R = <ArchonHunt as Queryable>::Return;

    #[rstest]
    fn test(archon_hunt_en: &str) {
        from_str::<R>(archon_hunt_en).unwrap();
    }

    #[rstest]
    fn test_ml(archon_hunt: &str) {
        from_str::<R>(archon_hunt).unwrap();
    }
}
