use super::{
    faction::Faction,
    macros::{
        enum_builder,
        model_builder,
    },
    mission_type::MissionType,
};

enum_builder! {
    :"Represents Relic tiers"
    Tier;
    :"Lith"
    Lith: 1,
    :"Meso"
    Meso: 2,
    :"Neo"
    Neo: 3,
    :"Axi"
    Axi: 4,
    :"Requiem"
    Requiem: 5,
    :"Omnia"
    Omnia: 6,
}

model_builder! {
    :"A Fissure Mission in which you can crack Void Relics"
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

    :"The Tier of the relic"
    pub tier: Tier = "tierNum",

    :"The i18n name of the enemy"
    pub enemy: String,

    :"The type of the enemy"
    pub faction: Faction = "enemyKey",

    :"Whether the fissure is a storm"
    pub is_storm: bool,

    :"Whether the the fissure is hard (Steel Path)"
    pub is_hard: bool
}

#[cfg(test)]
mod test {
    use super::Fissure;
    use crate::worldstate::{
        client::Client,
        error::Error,
    };

    #[tokio::test]
    async fn test_fissure() -> Result<(), Error> {
        let client = Client::new();

        match client.fetch::<Fissure>().await {
            Ok(_fissures) => Ok(()),
            Err(why) => Err(why),
        }
    }

    
    #[tokio::test]
    async fn test_fissure_ml() -> Result<(), Error> {
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_using_lang::<Fissure>(Language::ZH).await {
            Ok(_fissures) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
