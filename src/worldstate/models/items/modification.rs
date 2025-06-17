use chrono::NaiveDate;
use serde::Deserialize;

use super::{
    Introduced,
    LevelStat,
    Polarity,
    Rarity,
};

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
    pub base_drain: i64,

    pub compat_name: String,

    pub fusion_limit: i64,

    pub image_name: String,

    pub introduced: Introduced,

    pub is_exilus: bool,

    pub is_prime: bool,

    pub is_utility: bool,

    pub level_stats: Vec<LevelStat>,

    pub name: String,

    pub polarity: Polarity,

    pub rarity: Rarity,

    pub release_date: NaiveDate,

    pub tradable: bool,

    pub transmutable: bool,

    #[serde(rename = "type")]
    pub mod_type: ModType,

    pub unique_name: String,

    pub wikia_thumbnail: String,

    pub wikia_url: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum ModType {
    #[serde(rename = "Railjack Mod")]
    Railjack,
    #[serde(rename = "Necramech Mod")]
    Necramech,
    #[serde(rename = "Warframe Mod")]
    Warframe,
    #[serde(rename = "Secondary Mod")]
    Secondary,
    #[serde(rename = "Melee Mod")]
    Melee,
    #[serde(rename = "Companion Mod")]
    Companion,
    #[serde(rename = "Primary Mod")]
    Primary,
    #[serde(rename = "K-Drive Mod")]
    KDrive,
    #[serde(rename = "Riven Mod")]
    Riven,
    #[serde(rename = "Archwing Mod")]
    Archwing,
    #[serde(rename = "Arch-Melee Mod")]
    ArchMelee,
    #[serde(rename = "Arch-Gun Mod")]
    ArchGun,
    #[serde(rename = "Shotgun Mod")]
    Shotgun,
    #[serde(rename = "Creature Mod")]
    Creature,
    #[serde(rename = "Stance Mod")]
    Stance,
    #[serde(rename = "Parazon Mod")]
    Parazon,
    #[serde(rename = "Transmutation Mod")]
    Transmutation,
    #[serde(rename = "Peculiar Mod")]
    Peculiar,
    #[serde(rename = "Plexus Mod")]
    Plexus,
    #[serde(rename = "Posture Mod")]
    Posture,
}
