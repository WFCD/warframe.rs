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
