use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Deserializer,
};

use super::{
    base::TimedEvent,
    Faction,
    MissionType,
};

fn deserialize_expiry<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match DateTime::parse_from_rfc3339(s) {
        Ok(dt) => Ok(dt.with_timezone(&Utc)),
        Err(err) => {
            if let chrono::format::ParseErrorKind::OutOfRange
            | chrono::format::ParseErrorKind::Invalid = err.kind()
            {
                Ok(DateTime::<Utc>::MAX_UTC)
            } else {
                Err(serde::de::Error::custom(err.to_string()))
            }
        }
    }
}

// model_builder! {
//     :"Information about an arbitration"
//     Arbitration: "/arbitration",
//     rt = obj,
//     timed = true;

//     :"The i18n of the node"
//     pub node: String,

//     :"The name of the node"
//     pub node_key: String,

//     :"The i18n faction you are up against"
//     pub faction: Faction = "enemy",

//     :"The faction you are up against"
//     pub faction_key: Option<Faction> = "enemyKey",

//     :"The i18n type of the mission"
//     pub mission_type: String = "type",

//     :"The type of the mission"
//     pub mission_type_key: MissionType = "typeKey",

//     :"Whether this mission requires archwing"
//     pub archwing: bool,

//     :"Whether this mission requires sharkwing"
//     pub sharkwing: bool,
// }

// FROM THE MACRO
// Need custom deserializer for expiry
#[derive(Debug, serde::Deserialize, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
#[doc = "Information about an arbitration"]
pub struct Arbitration {
    #[doc = "The i18n of the node"]
    pub node: String,
    #[doc = "The name of the node"]
    pub node_key: String,
    #[serde(rename(deserialize = "enemy"))]
    #[doc = "The i18n faction you are up against"]
    pub faction: Faction,
    #[serde(rename(deserialize = "enemyKey"))]
    #[doc = "The faction you are up against"]
    pub faction_key: Option<Faction>,
    #[serde(rename(deserialize = "type"))]
    #[doc = "The i18n type of the mission"]
    pub mission_type: String,
    #[serde(rename(deserialize = "typeKey"))]
    #[doc = "The type of the mission"]
    pub mission_type_key: MissionType,
    #[doc = "Whether this mission requires archwing"]
    pub archwing: bool,
    #[doc = "Whether this mission requires sharkwing"]
    pub sharkwing: bool,
    activation: chrono::DateTime<chrono::Utc>,

    #[serde(deserialize_with = "deserialize_expiry")]
    expiry: chrono::DateTime<chrono::Utc>,
}

impl Arbitration {
    /// Whether the arbitration is still valid.
    pub fn is_valid(&self) -> bool {
        self.expiry() != DateTime::<Utc>::MAX_UTC
    }
}

impl crate::ws::TimedEvent for Arbitration {
    #[doc = "Returns the time when this event began"]
    fn activation(&self) -> chrono::DateTime<chrono::Utc> {
        self.activation
    }
    #[doc = "Returns the time when this event ends"]
    fn expiry(&self) -> chrono::DateTime<chrono::Utc> {
        self.expiry
    }
}
impl crate::ws::Endpoint for Arbitration {
    fn endpoint_en() -> &'static str {
        "https://api.warframestat.us/pc/arbitration/?language=en"
    }
    #[cfg(feature = "multilangual")]
    fn endpoint(language: crate::ws::Language) -> String {
        format!(
            "https://api.warframestat.us/pc/arbitration/?language={}",
            <crate::ws::Language as Into<&'static str>>::into(language),
        )
    }
}
impl crate::ws::Queryable for Arbitration {
    type Return = Arbitration;
}

impl crate::ws::TypeDocumentation for Arbitration {
    fn docs() -> &'static str {
        "Information about an arbitration"
    }
}

#[cfg(test)]
mod test {
    use super::Arbitration;
    use crate::worldstate::{
        client::Client,
        error::ApiError,
    };

    #[tokio::test]
    async fn test_arbitration() -> Result<(), ApiError> {
        let client = Client::new();

        match client.fetch::<Arbitration>().await {
            Ok(_arbitration) => Ok(()),
            Err(why) => {
                if let ApiError::ApiError(error) = why {
                    if error.code == 404 {
                        Ok(())
                    } else {
                        Err(ApiError::ApiError(error))
                    }
                } else {
                    Err(why)
                }
            }
        }
    }

    #[cfg(feature = "multilangual")]
    #[tokio::test]
    async fn test_arbitration_ml() -> Result<(), ApiError> {
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_using_lang::<Arbitration>(Language::ZH).await {
            Ok(_arbitration) => Ok(()),
            Err(why) => {
                if let ApiError::ApiError(error) = why {
                    if error.code == 404 {
                        Ok(())
                    } else {
                        Err(ApiError::ApiError(error))
                    }
                } else {
                    Err(why)
                }
            }
        }
    }
}
