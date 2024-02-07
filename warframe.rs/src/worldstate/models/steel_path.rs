use super::macros::model_builder;

model_builder! {
    :"An Item in Teshin's Shop"
    SteelPathShopItem,
    rt = obj,
    timed = false;

    :"The i18n Name of the Item"
    pub name: String,

    :"The amount of Steel Essence this Item costs"
    pub cost: i32,
}

model_builder! {
    :"Data about the missions for the current sortie"
    SteelPath: "/steelPath",
    rt = obj,
    timed = true;

    :"The current weekly offer"
    pub current_offer: SteelPathShopItem = "currentReward",

    :"The Rotation of Items Teshin offers"
    pub rotation: Vec<SteelPathShopItem>,

    :"Items that are always available"
    pub evergreens: Vec<SteelPathShopItem>
}
