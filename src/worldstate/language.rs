//! Adds an enum that represents the different languages that are supported by the API

/// An enumeration representing various supported languages.
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
        use Language::*;
        match value {
            DE => "de",
            ES => "es",
            FR => "fr",
            IT => "it",
            KO => "ko",
            PL => "pl",
            PT => "pt",
            RU => "ru",
            ZH => "zh",
            EN => "en",
            UK => "uk",
        }
        .into()
    }
}

impl From<Language> for &'static str {
    fn from(value: Language) -> Self {
        use Language::*;
        match value {
            DE => "de",
            ES => "es",
            FR => "fr",
            IT => "it",
            KO => "ko",
            PL => "pl",
            PT => "pt",
            RU => "ru",
            ZH => "zh",
            EN => "en",
            UK => "uk",
        }
    }
}
