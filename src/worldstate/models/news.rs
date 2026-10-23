#![allow(clippy::struct_excessive_bools)]

use std::collections::HashMap;

use chrono::{
    DateTime,
    Utc,
};
use warframe_macros::model;

use crate::worldstate::Language;

/// A news item
#[model(endpoint = "/news", return_style = Array)]
pub struct News {
    /// The id of the News
    pub id: String,

    /// The message associated to the News
    pub message: String,

    /// The link to the image associated with the News
    pub image_link: String,

    /// Whether the News are prioritized
    #[serde(default)]
    pub priority: bool,

    /// Whether the News are related to an update
    #[serde(default)]
    pub update: bool,

    /// Whether the News are related to a stream
    #[serde(default)]
    pub stream: bool,

    /// Whether the News are related to a Prime Access
    #[serde(default)]
    pub prime_access: bool,

    /// The date the News were posted
    pub date: DateTime<Utc>,

    /// The time when the News become active
    pub activation: Option<DateTime<Utc>>,

    /// The time when the News expire
    pub expiry: Option<DateTime<Utc>>,

    /// The translations of the News message
    pub translations: HashMap<Language, String>,

    /// The link to the News
    pub link: String,

    /// Whether the News are mobile-only
    #[serde(default)]
    pub mobile_only: bool,
}

#[cfg(test)]
mod test_news {
    use rstest::rstest;
    use serde_json::from_str;

    use super::News;
    use crate::worldstate::Queryable;

    type R = <News as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/news.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
