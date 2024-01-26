pub enum Language {
    DE,
    ES,
    FR,
    IT,
    KO,
    PL,
    PT,
    RU,
    ZH,
    EN,
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
