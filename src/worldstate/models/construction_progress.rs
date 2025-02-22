use warframe_macros::model;

use super::base::deserialize_f32_from_string;

/// Construction percentages for showing how far constructed the enemy fleets are
#[model(endpoint = "/constructionProgress", return_style = Object)]
pub struct ConstructionProgress {
    /// The progress of the Fomorian
    #[serde(deserialize_with = "deserialize_f32_from_string")]
    pub fomorian_progress: f32,

    /// The progress of the Razorback
    #[serde(deserialize_with = "deserialize_f32_from_string")]
    pub razorback_progress: f32,

    /// No clue what this is tbh
    #[serde(deserialize_with = "deserialize_f32_from_string")]
    pub unknown_progress: f32,
}

#[cfg(test)]
mod test {
    use super::ConstructionProgress;
    use crate::worldstate::{
        client::Client,
        error::Error,
    };

    #[tokio::test]
    async fn test_constructionprogress() -> Result<(), Error> {
        let client = Client::new();

        match client.fetch::<ConstructionProgress>().await {
            Ok(_constructionprogress) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[tokio::test]
    async fn test_constructionprogress_ml() -> Result<(), Error> {
        use crate::worldstate::language::Language;

        let client = Client::new();

        match client
            .fetch_using_lang::<ConstructionProgress>(Language::ZH)
            .await
        {
            Ok(_constructionprogress) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
