use serde::Deserialize;

use super::{
    focus_school::FocusSchool,
    profile::deserialize_oid,
};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutPreset {
    /// focus_school
    pub focus_school: Option<FocusSchool>,

    /// preset_icon
    pub preset_icon: String,

    /// favorite
    pub favorite: bool,

    #[serde(rename = "n")]
    /// name, only present if set
    pub name: Option<String>,

    #[serde(rename = "s")]
    /// warframe
    pub warframe: LoadOutPresetItem,

    #[serde(rename = "l")]
    /// primary
    pub primary: LoadOutPresetItem,

    #[serde(rename = "p")]
    /// secondary
    pub secondary: LoadOutPresetItem,

    #[serde(rename = "m")]
    /// melee
    pub melee: LoadOutPresetItem,

    #[serde(rename = "h")]
    // TODO: What is this?
    /// h
    pub h: Option<LoadOutPresetItem>,

    #[serde(rename = "a")]
    // TODO: What is this?
    /// a
    pub a: Option<LoadOutPresetItem>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutPresetItem {
    #[serde(deserialize_with = "deserialize_oid")]
    /// item_id
    pub item_id: String,

    #[serde(rename = "mod")]
    /// mod_loadout
    pub mod_loadout: u8,

    #[serde(rename = "cus")]
    /// customization_loadout
    pub customization_loadout: u8,

    #[serde(rename = "hide", default = "bool::default")]
    /// hide, only present in API if true
    pub hide: bool,
}
