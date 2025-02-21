use super::{
    macros::model_builder,
    Mission,
    RewardType,
};

model_builder! {
    :"An alert in Warframe"
    Alert: "/alerts",
    rt = array,
    timed = false;

    :"ID of this event"
    pub id: String,

    :"The mission associated with the alert"
    pub mission: Mission,

    :"The reward type of the alert"
    pub reward_types: Vec<RewardType>
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
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_using_lang::<Alert>(Language::ZH).await {
            Ok(_alerts) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
