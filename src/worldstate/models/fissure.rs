use super::enemy::Enemy;
use super::macros::{enum_builder, model_builder};
use super::mission_type::MissionType;

enum_builder! {
    Tier;
    Lith: 1,
    Meso: 2,
    Neo: 3,
    Axi: 4,
    Requiem: 5,
}

model_builder! {
    Fissure: "/fissures",
    rt = array,
    timed = true;

    :"The id of the fissure"
    pub id: String,

    :"The i18n of the mission"
    pub mission_type: String,

    :"The type of the mission"
    pub mission_key: MissionType,

    :"The i18n of the node"
    pub node: String,

    :"The name of the node"
    pub node_key: String,

    :"The tier i18n of the relic"
    pub tier_name: String = "tier",

    // :"The Tier of the relic"
    // pub tier: Tier = "tierNum",

    :"The i18n name of the enemy"
    pub enemy: String,

    :"The type of the enemy"
    pub enemy_key: Enemy,

    :"Whether the fissure is a storm"
    pub is_storm: bool,

    :"Whether the the fissure is hard (Steel Path)"
    pub is_hard: bool
}

#[cfg(test)]
mod test {
    use super::Fissure;
    use crate::worldstate::{client::Client, error::ApiError};

    #[cfg(not(feature = "multilangual"))]
    #[tokio::test]
    async fn test_fissure() -> Result<(), ApiError> {
        let client = Client::new();

        match client.fetch_arr::<Fissure>().await {
            Ok(_fissures) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[cfg(feature = "multilangual")]
    #[tokio::test]
    async fn test_fissure() -> Result<(), ApiError> {
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_arr::<Fissure>(Language::ZH).await {
            Ok(_fissures) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
