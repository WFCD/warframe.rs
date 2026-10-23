use serde_with::serde_as;
use warframe_macros::model;

/// An Item in Teshin's Shop
#[model]
pub struct SteelPathShopItem {
    /// The i18n Name of the Item
    pub name: String,

    /// The amount of Steel Essence this Item costs
    pub cost: i32,
}

/// Steel Path incursion data
#[model(timed)]
pub struct SteelPathIncursion {
    /// Unique identifier for the incursion
    pub id: String,
}

/// Data about steel path offerings
#[serde_as]
#[model(endpoint = "/steelPath", return_style = Object, timed)]
pub struct SteelPath {
    /// The current weekly offer
    #[serde(rename = "currentReward")]
    pub current_offer: SteelPathShopItem,

    /// Human-readable time remaining
    pub remaining: Option<String>,

    /// The Rotation of Items Teshin offers
    pub rotation: Vec<SteelPathShopItem>,

    /// Items that are always available
    pub evergreens: Vec<SteelPathShopItem>,

    /// Current Steel Path incursions
    #[serde_as(deserialize_as = "serde_with::OneOrMany<_>")]
    #[serde(default)]
    pub incursions: Vec<SteelPathIncursion>,
}

#[cfg(test)]
mod test_steel_path {
    use rstest::rstest;
    use serde_json::from_str;

    use super::SteelPath;
    use crate::worldstate::Queryable;

    type R = <SteelPath as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/steel_path.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
