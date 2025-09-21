//! Adds an enum that represents the different languages that are supported by the API

use core::str;

use warframe_macros::model;

/// An enumeration representing various supported languages.
#[model]
#[derive(Ord)]
#[serde(rename_all = "lowercase")]
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
    /// Turkish (`TR`)
    TR,
    /// Japanese (`JA`)
    JA,
    /// Traditional Chinese (`TC`)
    TC,
    /// Thai (`TH`)
    TH,
}
