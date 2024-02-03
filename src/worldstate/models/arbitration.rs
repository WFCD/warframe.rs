use super::{macros::model_builder, Faction, MissionType};

model_builder! {
    :"Information about an arbitration"
    Arbitration: "/arbitration",
    rt = obj,
    timed = true;

    :"ID of this event"
    pub id: String,

    :"The i18n of the node"
    pub node: String,

    :"The name of the node"
    pub node_key: String,

    :"The i18n faction you are up against"
    pub faction: Faction = "enemy",

    :"The faction you are up against"
    pub faction_key: Option<Faction> = "enemyKey",

    :"The i18n type of the mission"
    pub mission_type: String = "type",

    :"The type of the mission"
    pub mission_type_key: MissionType = "typeKey",

    :"Whether this mission requires archwing"
    pub archwing: bool,

    :"Whether this mission requires sharkwing"
    pub sharkwing: bool,
}

#[cfg(test)]
mod test {
    use super::Arbitration;
    use crate::worldstate::{client::Client, error::ApiError};

    #[tokio::test]
    async fn test_arbitration() -> Result<(), ApiError> {
        let client = Client::new();

        match client.fetch::<Arbitration>().await {
            Ok(_arbitration) => Ok(()),
            Err(why) => match why {
                ApiError::NotFound(_) => Ok(()),
                why => Err(why),
            },
        }
    }

    #[cfg(feature = "multilangual")]
    #[tokio::test]
    async fn test_arbitration_ml() -> Result<(), ApiError> {
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_using_lang::<Arbitration>(Language::ZH).await {
            Ok(_arbitration) => Ok(()),
            Err(why) => match why {
                ApiError::NotFound(_) => Ok(()),
                why => Err(why),
            },
        }
    }
}
