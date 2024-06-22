use {
    super::{
        error::ApiError,
        models::{
            item::{Item, ItemsPayload},
            item_info::{ItemInfo, ItemInfoPayload},
            orders::{Order, OrderPayload},
            statistic_item::{StatisticItem, StatisticItemPayload},
        },
    },
    std::fmt::Display,
};

#[derive(Default, Debug, Clone)]
pub struct Client {
    session: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Client {
    pub async fn item_statistics(&self, item_url: impl Display) -> Result<StatisticItem, ApiError> {
        let response = self
            .session
            .get(format!(
                "https://api.warframe.market/v1/items/{item_url}/statistics"
            ))
            .send()
            .await?;

        if response.status().is_success() {
            let json_result = response.json::<StatisticItemPayload>().await?;
            Ok(json_result.payload)
        } else {
            Err(response.status().into())
        }
    }

    pub async fn item_info(&self, item_url: impl Display) -> Result<ItemInfo, ApiError> {
        let response = self
            .session
            .get(format!("https://api.warframe.market/v1/items/{item_url}"))
            .send()
            .await?;

        if response.status().is_success() {
            let json_result = response.json::<ItemInfoPayload>().await?;
            Ok(json_result.payload.item)
        } else {
            Err(response.status().into())
        }
    }

    pub async fn items(&self) -> Result<Vec<Item>, ApiError> {
        let response = self
            .session
            .get("https://api.warframe.market/v1/items")
            .send()
            .await?;

        if response.status().is_success() {
            let json_result = response.json::<ItemsPayload>().await?;
            Ok(json_result.payload.items)
        } else {
            Err(response.status().into())
        }
    }

    pub async fn orders(&self, item_url: impl Display) -> Result<Vec<Order>, ApiError> {
        let response = self
            .session
            .get(format!(
                "https://api.warframe.market/v1/items/{item_url}/orders"
            ))
            .send()
            .await?;

        if response.status().is_success() {
            let json_result = response.json::<OrderPayload>().await?;
            Ok(json_result.payload.orders)
        } else {
            Err(response.status().into())
        }
    }
}
