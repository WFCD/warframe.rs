use chrono::Utc;

use super::macros::model_builder;

type DateTime = chrono::DateTime<Utc>;

model_builder! {
    :"[POSSIBLY UNSTABLE] Any current modifiers applied to all users, such as double drops, double XP, etc."
    GlobalUpgrade: "/globalUpgrades",
    rt = array,
    timed = true;

    :"The start of the upgrade"
    pub start: DateTime,

    :"The end of the upgrade"
    pub end: DateTime,

    :"What kind of upgrade"
    pub upgrade: String,

    :"Operation descriptor"
    pub operation: String,

    :"Symbol corresponding to operation"
    pub operation_symbol: String,

    :"Value corresponding to performing the operation"
    pub upgrade_operation_value: i32,

    :"Whether the upgrade has expired"
    pub expired: bool,

    :"Formatted short string designating when the upgrade will expire"
    pub eta: String,
}

#[cfg(test)]
mod test {
    use super::GlobalUpgrade;
    use crate::worldstate::{client::Client, error::ApiError};

    #[cfg(not(feature = "multilangual"))]
    #[tokio::test]
    async fn test_globalupgrade() -> Result<(), ApiError> {
        let client = Client::new();

        match client.fetch::<GlobalUpgrade>().await {
            Ok(_globalupgrades) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[cfg(feature = "multilangual")]
    #[tokio::test]
    async fn test_globalupgrade_ml() -> Result<(), ApiError> {
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_using_lang::<GlobalUpgrade>(Language::ZH).await {
            Ok(_globalupgrades) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
