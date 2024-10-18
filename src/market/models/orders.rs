use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct OrderPayload {
    pub(crate) payload: Payload,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct Payload {
    pub(crate) orders: Vec<Order>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
/// Platform is missing here. Platform is always "pc"
pub struct Order {
    pub platinum: i64,

    pub order_type: OrderType,

    pub quantity: i64,

    pub user: User,

    pub creation_date: DateTime<Utc>,

    pub last_update: DateTime<Utc>,

    pub visible: bool,

    pub id: String,

    pub region: Region,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(rename_all = "kebab-case")]
pub enum Region {
    De,
    En,
    Es,
    Fr,
    Ko,
    Pl,
    Pt,
    Ru,
    Uk,
    Cs,

    #[serde(rename = "zh-hans")]
    ZhHans,

    #[serde(rename = "zh-hant")]
    ZhHant,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct User {
    pub reputation: i64,

    pub locale: Region,

    pub avatar: Option<String>,

    pub last_seen: DateTime<Utc>,

    pub ingame_name: String,

    pub id: String,

    pub region: Region,

    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Ingame,
    Offline,
    Online,
}

#[cfg(test)]
mod test {
    use crate::market::{
        client::Client,
        error::ApiError,
    };

    #[tokio::test]
    async fn test_orders() -> Result<(), ApiError> {
        let client = Client::new();
        let _ = client.orders("mirage_prime_set").await?;
        Ok(())
    }
}
