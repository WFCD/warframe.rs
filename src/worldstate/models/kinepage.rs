use std::collections::HashMap;

use chrono::{
    DateTime,
    Utc,
};
use warframe_macros::model;

use crate::worldstate::Language;

/// The Kinepage message from 1999
#[model(endpoint = "/kinepage", return_style = Object)]
pub struct Kinepage {
    /// When this message was posted
    pub timestamp: DateTime<Utc>,

    /// The message content
    pub message: String,

    /// Translations of the message keyed by language code
    pub translations: HashMap<Language, String>,
}

#[cfg(test)]
mod test_kinepage {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Kinepage;
    use crate::worldstate::Queryable;

    type R = <Kinepage as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/kinepage.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
