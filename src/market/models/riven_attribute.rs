use serde::Deserialize;

use super::{
    i18n::I18N,
    impl_queryable,
};

impl_queryable!(RivenAttribute, Array, "/riven/attributes");

/// Represents the `/riven/attributes` endpoint
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RivenAttribute {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub group: Option<String>,
    pub prefix: String,
    pub suffix: String,
    #[serde(default)]
    pub exclusive_to: Vec<String>,
    #[serde(default)]
    pub unit: Unit,
    #[serde(default)]
    pub positive_is_negative: bool,
    #[serde(default)]
    pub positive_only: bool,
    #[serde(default)]
    pub negative_only: bool,
    pub i18n: I18N<RivenAttributeI18N>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RivenAttributeI18N {
    pub name: String,
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
pub enum Unit {
    /// A percentage value - such as 200% Damage
    Percent,
    /// A multiplier - such a 1.43x Damage to Grineer
    Multiply,
    /// Seconds - such +5s Combo Duration
    Seconds,

    /// A flat value - such as +23.4 Initial Combo Count
    #[default]
    Bare,
}

#[cfg(test)]
mod test {
    use super::RivenAttribute;
    use crate::market::{
        Queryable,
        models::ResponseBase,
    };

    #[rstest::rstest]
    fn riven_attribute(
        #[files("src/market/models/fixtures/riven_attribute.json")]
        #[mode = str]
        json: &str,
    ) -> Result<(), serde_json::Error> {
        serde_json::from_str::<ResponseBase<<RivenAttribute as Queryable>::Data>>(json)?;

        Ok(())
    }
}
