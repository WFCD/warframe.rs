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

    /// A string describing this element
    pub as_string: String,

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
    use crate::worldstate::{
        Queryable,
        fixtures::news::{
            news,
            news_en,
        },
    };

    type R = <News as Queryable>::Return;

    #[rstest]
    fn test(news_en: &str) {
        from_str::<R>(news_en).unwrap();
    }

    #[rstest]
    fn test_ml(news: &str) {
        from_str::<R>(news).unwrap();
    }
}
