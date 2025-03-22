use warframe_macros::model;

/// An Item in Teshin's Shop
#[model]
pub struct SteelPathShopItem {
    /// The i18n Name of the Item
    pub name: String,

    /// The amount of Steel Essence this Item costs
    pub cost: i32,
}

/// Data about the missions for the current sortie
#[model(endpoint = "/steelPath", return_style = Object, timed)]
pub struct SteelPath {
    /// The current weekly offer
    #[serde(rename = "currentReward")]
    pub current_offer: SteelPathShopItem,

    /// The Rotation of Items Teshin offers
    pub rotation: Vec<SteelPathShopItem>,

    /// Items that are always available
    pub evergreens: Vec<SteelPathShopItem>,
}

#[cfg(test)]
mod test_steel_path {
    use rstest::rstest;
    use serde_json::from_str;

    use super::SteelPath;
    use crate::worldstate::{
        Queryable,
        fixtures::steel_path::{
            steel_path,
            steel_path_en,
        },
    };

    type R = <SteelPath as Queryable>::Return;

    #[rstest]
    fn test(steel_path_en: &str) {
        from_str::<R>(steel_path_en).unwrap();
    }

    #[rstest]
    fn test_ml(steel_path: &str) {
        from_str::<R>(steel_path).unwrap();
    }
}
