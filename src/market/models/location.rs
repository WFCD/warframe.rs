use serde::Deserialize;

use super::{
    i18n::I18N,
    impl_queryable,
};

impl_queryable!(Location, Array, "/locations");

/// Represents the `/locations` endpoint
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    #[serde(default)]
    pub faction: Faction,
    pub min_level: Option<i32>,
    pub max_level: Option<i32>,
    pub i18n: I18N<LocationI18N>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LocationI18N {
    pub node_name: String,
    pub system_name: Option<String>,
    pub icon: String,
    pub thumb: String,
}

#[derive(
    Debug,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Hash,
    derive_more::Display,
    Default,
)]
#[serde(rename_all = "lowercase")]
pub enum Faction {
    Infested,
    Corpus,
    Grineer,
    Corrupted,
    Ropalolyst,
    Murmur,
    #[default]
    Unknown,
}

#[cfg(test)]
mod test {
    use super::Location;
    use crate::market::{
        Queryable,
        models::ResponseBase,
    };

    #[rstest::rstest]
    fn location(
        #[files("src/market/models/fixtures/location.json")]
        #[mode = str]
        json: &str,
    ) -> Result<(), serde_json::Error> {
        serde_json::from_str::<ResponseBase<<Location as Queryable>::Data>>(json)?;

        Ok(())
    }
}
