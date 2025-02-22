use warframe_macros::model;

use super::{
    Faction,
    MissionType,
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
mod test {
    use super::ArchonHunt;
    use crate::worldstate::{
        client::Client,
        error::Error,
    };

    #[tokio::test]
    async fn test_archonhunt() -> Result<(), Error> {
        let client = Client::new();

        match client.fetch::<ArchonHunt>().await {
            Ok(_archonhunt) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[tokio::test]
    async fn test_archonhunt_ml() -> Result<(), Error> {
        use crate::worldstate::language::Language;

        let client = Client::new();

        match client.fetch_using_lang::<ArchonHunt>(Language::ZH).await {
            Ok(_archonhunt) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
