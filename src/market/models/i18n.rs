use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
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
    En,
}

pub type I18N<T> = HashMap<Language, T>;
