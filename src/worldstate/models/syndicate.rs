#![allow(clippy::doc_markdown)]

use super::faction::SyndicateFaction;

/// A Syndicate in Warframe, with fallback for unrecognized syndicate keys
#[derive(
    Debug,
    PartialEq,
    PartialOrd,
    Clone,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::From,
    derive_more::TryInto,
)]
pub enum Syndicate {
    /// A known syndicate
    Known(SyndicateFaction),
    /// An unrecognized syndicate
    Unknown(String),
}

impl<'de> serde::Deserialize<'de> for Syndicate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value =
            serde_json::Value::deserialize(deserializer).map_err(serde::de::Error::custom)?;

        if let Ok(v) = SyndicateFaction::deserialize(&value) {
            return Ok(Syndicate::Known(v));
        }

        match value.as_str() {
            Some(s) => Ok(Syndicate::Unknown(s.to_owned())),
            None => Ok(Syndicate::Unknown(value.to_string())),
        }
    }
}
