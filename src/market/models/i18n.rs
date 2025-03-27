use std::collections::HashMap;

use derive_more::derive::{
    Display,
    FromStr,
};
use serde::Deserialize;

#[derive(
    Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, FromStr, Display, Default,
)]
pub enum Language {
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "zh-hans")]
    ZhHans,
    #[serde(rename = "zh-hant")]
    ZhHant,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "uk")]
    Uk,
    #[serde(rename = "en")]
    #[default]
    En,
}

pub type I18N<T> = HashMap<Language, T>;
