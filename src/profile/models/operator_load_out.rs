use serde::Deserialize;

use super::color_load_out::ColorLoadOut;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct OperatorLoadOut {
    #[serde(default)]
    pub skins: Vec<String>,

    pub upgrades: Option<Vec<String>>,

    pub ability_override: Option<AbilityOverride>,

    #[serde(rename = "pricol")]
    pub primary_colors: Option<ColorLoadOut>,

    #[serde(rename = "eyecol")]
    pub eye_colors: Option<ColorLoadOut>,

    #[serde(rename = "sigcol")]
    pub sigil_colors: Option<ColorLoadOut>,

    #[serde(rename = "cloth")]
    pub cloth_colors: Option<ColorLoadOut>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AbilityOverride {
    pub ability: String,

    pub index: i32,
}
