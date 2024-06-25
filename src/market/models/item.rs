use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ItemsPayload {
    pub(crate) payload: Items,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Items {
    pub(crate) items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub thumb: String,

    pub item_name: String,

    pub url_name: String,

    pub id: String,

    #[serde(default)]
    pub vaulted: bool,
}

#[cfg(test)]
mod test {
    use crate::market::{client::Client, error::ApiError};

    #[tokio::test]
    async fn test_items() -> Result<(), ApiError> {
        let client = Client::new();
        let _ = client.items().await?;
        Ok(())
    }
}
