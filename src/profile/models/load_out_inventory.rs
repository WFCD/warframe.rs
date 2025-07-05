use serde::{
    Deserialize,
    Serialize,
};

use super::color_load_out::ColorLoadOut;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutInventory {
    pub weapon_skins: Vec<LoadOutInventoryItemType>,

    #[serde(rename = "Suits")]
    pub warframe: Vec<LoadOutInventoryItem<WarframeLoadOutInventoryItemConfig>>,

    #[serde(rename = "LongGuns")]
    pub primaries: Vec<LoadOutInventoryItem<LoadOutInventoryItemConfig>>,

    #[serde(rename = "Pistols")]
    pub secondaries: Vec<LoadOutInventoryItem<LoadOutInventoryItemConfig>>,

    pub melee: Vec<LoadOutInventoryItem<LoadOutInventoryItemConfig>>,

    #[serde(rename = "XPInfo")]
    pub xp_info: Vec<XPInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutInventoryItemType {
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutInventoryItem<Config> {
    pub item_type: String,

    pub configs: Vec<Config>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutInventoryItemConfig {
    pub name: Option<String>,

    #[serde(default)]
    pub skins: Vec<String>,

    #[serde(rename = "pricol")]
    pub primary_colors: Option<ColorLoadOut>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct WarframeLoadOutInventoryItemConfig {
    #[serde(flatten)]
    pub base: LoadOutInventoryItemConfig,

    #[serde(rename = "attcol")]
    pub attachment_colors: Option<ColorLoadOut>,

    #[serde(rename = "sigcol")]
    pub sigil_colors: Option<ColorLoadOut>,

    #[serde(rename = "syancol")]
    pub syandana_colors: Option<ColorLoadOut>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct XPInfo {
    pub item_type: String,

    #[serde(rename = "XP")]
    pub xp: u32,
}
