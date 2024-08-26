use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct ItemInfoPayload {
    pub(crate) payload: Payload,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct Payload {
    pub(crate) item: ItemInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct ItemInfo {
    pub id: String,

    pub items_in_set: Vec<ItemInSet>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct ItemInSet {
    pub thumb: String,

    pub trading_tax: i64,

    pub url_name: String,

    pub set_root: bool,

    pub ducats: i64,

    pub sub_icon: Option<String>,

    pub tags: Vec<String>,

    pub quantity_for_set: Option<i64>,

    pub icon: String,

    pub icon_format: String,

    pub mastery_level: i64,

    pub id: String,

    #[serde(rename = "en")]
    pub info: LanguageItem,
}

/// A Language Item
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct LanguageItem {
    /// item_name
    pub item_name: String,

    /// description
    pub description: String,

    /// wiki_link
    pub wiki_link: String,

    /// thumb
    pub thumb: String,

    /// icon
    pub icon: String,
    // drop: Vec<Option<serde_json::Value>>, // seems to be empty all the time
}

#[cfg(test)]
mod test {
    use crate::market::{client::Client, error::ApiError};

    #[tokio::test]
    async fn test_items() -> Result<(), ApiError> {
        let client = Client::new();
        let _ = client.item_info("mirage_prime_set").await?;
        Ok(())
    }
}
