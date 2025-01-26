use serde::Deserialize;
use super::color_load_out::ColorLoadOut;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct OperatorLoadOut {
    /// skins
    pub skins: Vec<String>,

    /// upgrades
    pub upgrades: Option<Vec<String>>,

    /// ability_override
    pub ability_override: Option<AbilityOverride>,

    #[serde(rename = "pricol")]
    /// primary_colors
    pub primary_colors: Option<ColorLoadOut>,

    #[serde(rename = "eyecol")]
    /// eye_colors
    pub eye_colors: Option<ColorLoadOut>,

    #[serde(rename = "sigcol")]
    /// sigil_colors
    pub sigil_colors: Option<ColorLoadOut>,

    #[serde(rename = "cloth")]
    /// cloth_colors
    pub cloth_colors: Option<ColorLoadOut>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AbilityOverride {
    /// ability
    pub ability: String,

    /// index
    pub index: i32,
}
