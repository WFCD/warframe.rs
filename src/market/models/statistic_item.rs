use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

use super::OrderType;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct StatisticItemPayload {
    pub(crate) payload: StatisticItem,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct StatisticItem {
    pub statistics_closed: StatisticsClosed,
    pub statistics_live: StatisticsLive,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct StatisticsClosed {
    #[serde(rename = "48hours")]
    pub last_48_hours: Vec<StatisticsClosed48Hour>,

    #[serde(rename = "90days")]
    pub last_90_days: Vec<StatisticsClosed48Hour>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct StatisticsClosed48Hour {
    pub datetime: DateTime<Utc>,

    pub volume: f64,

    pub min_price: f64,

    pub max_price: f64,

    pub open_price: f64,

    pub closed_price: f64,

    pub avg_price: f64,

    pub wa_price: f64,

    pub median: f64,

    pub moving_avg: Option<f64>,

    pub donch_top: f64,

    pub donch_bot: f64,

    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct StatisticsLive {
    #[serde(rename = "48hours")]
    pub last_48_hours: Vec<StatisticsLive48Hour>,

    #[serde(rename = "90days")]
    pub last_90_days: Vec<StatisticsLive48Hour>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct StatisticsLive48Hour {
    pub datetime: DateTime<Utc>,

    pub volume: f64,

    pub min_price: f64,

    pub max_price: f64,

    pub avg_price: f64,

    pub wa_price: f64,

    pub median: f64,

    pub order_type: OrderType,

    pub moving_avg: Option<f64>,

    pub id: String,
}

#[cfg(test)]
mod test {
    use crate::market::{
        client::Client,
        error::ApiError,
    };

    #[tokio::test]
    async fn test_statistics() -> Result<(), ApiError> {
        let client = Client::new();
        let _ = client.item_statistics("mirage_prime_set").await?;
        Ok(())
    }
}
