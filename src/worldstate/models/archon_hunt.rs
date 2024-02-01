use super::macros::model_builder;
use super::Faction;
use super::MissionType;

model_builder! {
    :"An archon hunt mission"
    ArchonHuntMission,
    rt = obj,
    timed = false;

    :"The i18n of the node"
    pub node: String,

    :"The name of the node"
    pub node_key: String,

    :"The i18n type of the mission"
    pub r#type: String,

    :"The type of the mission"
    pub type_key: MissionType,

    // Given by the API, but Archon Hunt missions cannot be nightmare
    // :"Whether the mission is a nightmare mission"
    // pub nightmare: bool,

    :"Whether the mission requires an archwing"
    pub archwing_required: bool,

    :"Whether the mission is a sharkwing mission"
    pub is_sharkwing: bool,

    pub advanced_spawners: Vec<String>,

    :"Items required to enter the mission"
    pub required_items: Vec<String>,

    :"Affectors of this mission"
    pub level_auras: Vec<String>,
}

model_builder! {
    :"An alert in Warframe"
    ArchonHunt: "/archonHunt",
    rt = obj,
    timed = true;

    :"ID of this event"
    pub id: String,

    :"The Archon you are up against"
    pub boss: String,

    :"The Reward Pool of this event"
    pub reward_pool: String,

    :"The i18n of the faction you are up against"
    pub faction: String,

    :"The faction you are up against"
    pub faction_key: Option<Faction>,

    :"The missions associated with this week's Archon Hunt"
    pub missions: [ArchonHuntMission; 3]
}

#[cfg(test)]
mod test {
    use super::ArchonHunt;
    use crate::worldstate::{client::Client, error::ApiError};

    #[cfg(not(feature = "multilangual"))]
    #[tokio::test]
    async fn test_archonhunt() -> Result<(), ApiError> {
        let client = Client::new();

        match client.fetch::<ArchonHunt>().await {
            Ok(_archonhunt) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[cfg(feature = "multilangual")]
    #[tokio::test]
    async fn test_archonhunt_ml() -> Result<(), ApiError> {
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_using_lang::<ArchonHunt>(Language::ZH).await {
            Ok(_archonhunt) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
