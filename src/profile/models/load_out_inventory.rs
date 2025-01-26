use serde::{Deserialize, Serialize};
use super::color_load_out::ColorLoadOut;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutInventory {
    /// weapon_skins
    pub weapon_skins: Vec<LoadOutInventoryItemType>,

    #[serde(rename = "Suits")]
    /// warframes
    pub warframe: Vec<LoadOutInventoryItem<WarframeLoadOutInventoryItemConfig>>,

    #[serde(rename = "LongGuns")]
    /// primaries
    pub primaries: Vec<LoadOutInventoryItem>,

    #[serde(rename = "Pistols")]
    /// secondaries
    pub secondaries: Vec<LoadOutInventoryItem>,

    /// melee
    pub melee: Vec<LoadOutInventoryItem>,

    #[serde(rename = "XPInfo")]
    /// xp_info
    pub xp_info: Vec<XPInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutInventoryItemType {
    /// item_type
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutInventoryItem<Config = LoadOutInventoryItemConfig> {
    /// item_type
    pub item_type: String,

    /// configs
    pub configs: Vec<Config>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutInventoryItemConfig {
    /// name
    pub name: Option<String>,

    /// skins
    pub skins: Vec<String>,

    #[serde(rename = "pricol")]
    /// primary_colors
    pub primary_colors: Option<ColorLoadOut>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct WarframeLoadOutInventoryItemConfig {
    #[serde(flatten)]
    pub base: LoadOutInventoryItemConfig,

    #[serde(rename = "attcol")]
    /// attachment_colors
    pub attachment_colors: Option<ColorLoadOut>,

    #[serde(rename = "sigcol")]
    /// sigil_colors
    pub sigil_colors: Option<ColorLoadOut>,

    #[serde(rename = "syancol")]
    /// syandana_colors
    pub syandana_colors: Option<ColorLoadOut>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct XPInfo {
    /// item_type
    pub item_type: String,

    #[serde(rename = "XP")]
    /// xp
    pub xp: u32,
}
