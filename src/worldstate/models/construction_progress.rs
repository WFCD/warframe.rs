use super::base::deserialize_f32_from_string;

use super::macros::model_builder;

model_builder! {
    :"Construction percentages for showing how far constructed the enemy fleets are"
    ConstructionProgress: "/constructionProgress",
    rt = obj,
    timed = false;

    pub fomorian_progress: f32 => "deserialize_f32_from_string",
    pub razorback_progress: f32 => "deserialize_f32_from_string",

    :"No clue what this is tbh"
    pub unknown_progress: f32 => "deserialize_f32_from_string",
}

#[cfg(test)]
mod test {
    use super::ConstructionProgress;
    use crate::worldstate::{client::Client, error::ApiError};

    #[cfg(not(feature = "multilangual"))]
    #[tokio::test]
    async fn test_constructionprogress() -> Result<(), ApiError> {
        let client = Client::new();

        match client.fetch::<ConstructionProgress>().await {
            Ok(_constructionprogress) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[cfg(feature = "multilangual")]
    #[tokio::test]
    async fn test_constructionprogress_ml() -> Result<(), ApiError> {
        use crate::worldstate::prelude::Language;

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
