//! Arcane type and utils

use serde::Deserialize;

use super::{
    Drop,
    LevelStat,
    Rarity,
};

/// An Arcane
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Arcane {
    pub drops: Vec<Drop>,

    pub image_name: String,

    pub level_stats: Vec<LevelStat>,

    pub masterable: bool,

    pub name: String,

    pub rarity: Rarity,

    pub tradable: bool,

    pub unique_name: String,
}
