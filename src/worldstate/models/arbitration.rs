use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Deserializer,
};
use warframe_macros::model;

use super::{
    Faction,
    MissionType,
    base::TimedEvent,
};

fn deserialize_expiry<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .or_else(|err| {
            if matches!(
                err.kind(),
                chrono::format::ParseErrorKind::OutOfRange
                    | chrono::format::ParseErrorKind::Invalid
            ) {
                Ok(DateTime::<Utc>::MAX_UTC)
            } else {
                Err(serde::de::Error::custom(err.to_string()))
            }
        })
}

/// Information about an arbitration
#[model(
    endpoint = "/arbitration",
    return_style = Object,
    timed,
    expiry(serde(deserialize_with = "deserialize_expiry")),
)]
pub struct Arbitration {
    /// The i18n of the node
    pub node: String,

    /// The name of the node
    pub node_key: String,

    /// The i18n faction you are up against
    #[serde(rename(deserialize = "enemy"))]
    pub faction: Faction,

    /// The faction you are up against
    #[serde(rename(deserialize = "enemyKey"))]
    pub faction_key: Option<Faction>,

    /// The i18n type of the mission
    #[serde(rename(deserialize = "type"))]
    pub mission_type: String,

    /// The type of the mission
    #[serde(rename(deserialize = "typeKey"))]
    pub mission_type_key: MissionType,

    /// Whether this mission requires archwing
    pub archwing: bool,

    /// Whether this mission requires sharkwing
    pub sharkwing: bool,
}

impl Arbitration {
    /// Whether the arbitration is still valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.expiry() != DateTime::<Utc>::MAX_UTC
    }
}

#[cfg(test)]
mod test {
    use super::Arbitration;
    use crate::worldstate::{
        client::Client,
        error::Error,
    };

    #[tokio::test]
    async fn test_arbitration() -> Result<(), Error> {
        let client = Client::new();

        match client.fetch::<Arbitration>().await {
            Ok(_arbitration) => Ok(()),
            Err(why) => {
                if let Error::ApiError(error) = why {
                    if error.code == 404 {
                        Ok(())
                    } else {
                        Err(Error::ApiError(error))
                    }
                } else {
                    Err(why)
                }
            }
        }
    }

    #[tokio::test]
    async fn test_arbitration_ml() -> Result<(), Error> {
        use crate::worldstate::language::Language;

        let client = Client::new();

        match client.fetch_using_lang::<Arbitration>(Language::ZH).await {
            Ok(_arbitration) => Ok(()),
            Err(why) => {
                if let Error::ApiError(error) = why {
                    if error.code == 404 {
                        Ok(())
                    } else {
                        Err(Error::ApiError(error))
                    }
                } else {
                    Err(why)
                }
            }
        }
    }
}
