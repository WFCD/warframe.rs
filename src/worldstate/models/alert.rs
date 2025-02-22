use warframe_macros::model;

use super::{
    Mission,
    RewardType,
};

/// An alert in Warframe
#[model(endpoint = "/alerts", return_style = Array)]
pub struct Alert {
    /// ID of this event
    pub id: String,

    /// The mission associated with the alert
    pub mission: Mission,

    /// The reward type of the alert
    pub reward_types: Vec<RewardType>,
}

#[cfg(test)]
mod test {
    use super::Alert;
    use crate::worldstate::{
        client::Client,
        error::Error,
    };

    #[tokio::test]
    async fn test_alert() -> Result<(), Error> {
        let client = Client::new();

        match client.fetch::<Alert>().await {
            Ok(_alerts) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[tokio::test]
    async fn test_alert_ml() -> Result<(), Error> {
        use crate::worldstate::language::Language;

        let client = Client::new();

        match client.fetch_using_lang::<Alert>(Language::ZH).await {
            Ok(_alerts) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
