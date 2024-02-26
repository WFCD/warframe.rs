use chrono::{DateTime, Utc};

use super::macros::model_builder;

model_builder! {
    :"A news item"
    News: "/news",
    rt = array,
    timed = false;

    :"The id of the News"
    pub id: String,

    :"The message associated to the News"
    pub message: String,

    :"The link to the image associated with the News"
    pub image_link: String,

    :"Whether the News are prioritized"
    pub priority: bool,

    :"Whether the News are related to an update"
    pub update: bool,

    :"Whether the News are related to a stream"
    pub stream: bool,

    :"A string describing this element"
    pub as_string: String,

    :"The date the News were posted"
    pub date: DateTime<Utc>,

    :"When the event that is associated with the News begins"
    pub start_date: Option<DateTime<Utc>>,

    :"When the event that is associated with the News ends"
    pub end_date: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod test {
    use super::News;
    use crate::worldstate::{client::Client, error::ApiError};

    #[cfg(not(feature = "multilangual"))]
    #[tokio::test]
    async fn test_news() -> Result<(), ApiError> {
        let client = Client::new();

        match client.fetch_arr::<News>().await {
            Ok(_newss) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[cfg(feature = "multilangual")]
    #[tokio::test]
    async fn test_news_ml() -> Result<(), ApiError> {
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_arr_using_lang::<News>(Language::ZH).await {
            Ok(_newss) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
