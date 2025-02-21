//! Adds an enum that represents the different languages that are supported by the API

use core::str;
use std::fmt::Display;

/// An enumeration representing various supported languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Language {
    /// German (`DE`)
    DE,
    /// Spanish (`ES`)
    ES,
    /// French (`FR`)
    FR,
    /// Italian (`IT`)
    IT,
    /// Korean (`KO`)
    KO,
    /// Polish (`PL`)
    PL,
    /// Portuguese (`PT`)
    PT,
    /// Russian (`RU`)
    RU,
    /// Chinese (`ZH`)
    ZH,
    /// English (`EN`)
    EN,
    /// Ukrainian (`UK`)
    UK,
}

impl From<Language> for String {
    fn from(value: Language) -> Self {
        match value {
            Language::DE => "de",
            Language::ES => "es",
            Language::FR => "fr",
            Language::IT => "it",
            Language::KO => "ko",
            Language::PL => "pl",
            Language::PT => "pt",
            Language::RU => "ru",
            Language::ZH => "zh",
            Language::EN => "en",
            Language::UK => "uk",
        }
        .into()
    }
}

impl From<Language> for &'static str {
    fn from(value: Language) -> Self {
        match value {
            Language::DE => "de",
            Language::ES => "es",
            Language::FR => "fr",
            Language::IT => "it",
            Language::KO => "ko",
            Language::PL => "pl",
            Language::PT => "pt",
            Language::RU => "ru",
            Language::ZH => "zh",
            Language::EN => "en",
            Language::UK => "uk",
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&'static str>::from(*self))
    }
}
