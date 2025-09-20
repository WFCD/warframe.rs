use chrono::{
    DateTime,
    Utc,
};
use warframe_macros::model;

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
    pub priority: bool,

    /// Whether the News are related to an update
    pub update: bool,

    /// Whether the News are related to a stream
    pub stream: bool,

    /// The date the News were posted
    pub date: DateTime<Utc>,

    /// When the event that is associated with the News begins
    pub start_date: Option<DateTime<Utc>>,

    /// When the event that is associated with the News ends
    pub end_date: Option<DateTime<Utc>>,
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
        #[files("src/worldstate/models/fixtures/news.json")]
        #[mode = str]
        news_en: &str,
    ) {
        from_str::<R>(news_en).unwrap();
    }
}
