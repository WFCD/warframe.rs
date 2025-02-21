use super::{
    base::deserialize_f32_from_string,
    macros::model_builder,
};

model_builder! {
    :"Construction percentages for showing how far constructed the enemy fleets are"
    ConstructionProgress: "/constructionProgress",
    rt = obj,
    timed = false;

    :"The progress of the Fomorian"
    pub fomorian_progress: f32 => "deserialize_f32_from_string",

    :"The progress of the Razorback"
    pub razorback_progress: f32 => "deserialize_f32_from_string",

    :"No clue what this is tbh"
    pub unknown_progress: f32 => "deserialize_f32_from_string",
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
