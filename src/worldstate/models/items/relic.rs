use serde::Deserialize;

use super::Category;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Relic {
    pub category: Category,

    pub description: String,

    pub image_name: String,

    pub locations: Vec<Option<serde_json::Value>>,

    pub market_info: MarketInfo,

    pub name: String,

    pub rewards: Vec<Reward>,

    pub tradable: bool,

    #[serde(rename = "type")]
    pub relic_type: String,

    pub unique_name: String,

    pub vaulted: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MarketInfo {
    pub id: String,

    pub url_name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Reward {
    pub rarity: String,

    pub chance: f64,

    pub item: RewardItem,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RewardItem {
    pub name: String,

    pub unique_name: String,

    pub warframe_market: MarketInfo,
}

#[tokio::test]
async fn test_relic_query() -> Result<(), Box<dyn std::error::Error>> {
    let _relic = reqwest::get("https://api.warframestat.us/items/axi%20a1/")
        .await?
        .json::<Relic>()
        .await?;
    Ok(())
}
