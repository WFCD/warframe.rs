use serde::Deserialize;

use super::{
    focus_school::FocusSchool,
    profile::deserialize_oid,
};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutPreset {
    pub focus_school: Option<FocusSchool>,

    pub preset_icon: String,

    pub favorite: bool,

    #[serde(rename = "n")]
    pub name: Option<String>,

    #[serde(rename = "s")]
    pub warframe: LoadOutPresetItem,

    #[serde(rename = "l")]
    pub primary: LoadOutPresetItem,

    #[serde(rename = "p")]
    pub secondary: LoadOutPresetItem,

    #[serde(rename = "m")]
    pub melee: LoadOutPresetItem,

    #[serde(rename = "h")]
    // TODO: What is this?
    pub h: Option<LoadOutPresetItem>,

    #[serde(rename = "a")]
    // TODO: What is this?
    pub a: Option<LoadOutPresetItem>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutPresetItem {
    #[serde(deserialize_with = "deserialize_oid")]
    pub item_id: String,

    #[serde(rename = "mod")]
    pub mod_loadout: u8,

    #[serde(rename = "cus")]
    pub customization_loadout: u8,

    #[serde(rename = "hide", default = "bool::default")]
    /// only present in API if true
    pub hide: bool,
}
