use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct ItemsPayload {
    pub(crate) payload: Items,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct Items {
    pub(crate) items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Item {
    /// thumb
    pub thumb: String,

    /// item_name
    pub item_name: String,

    /// url_name
    pub url_name: String,

    /// id
    pub id: String,

    #[serde(default)]
    /// vaulted
    pub vaulted: bool,
}

#[cfg(test)]
mod test {
    use crate::market::{client::Client, error::ApiError};

    #[tokio::test]
    async fn test_item() -> Result<(), ApiError> {
        let client = Client::new();
        let _ = client.items().await?;
        Ok(())
    }
}
