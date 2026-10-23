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
}

/// An Archon Hunt event
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
    use crate::worldstate::Queryable;

    type R = <ArchonHunt as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/archon_hunt.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
