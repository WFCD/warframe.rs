use serde::Deserialize;

use super::Category;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Quest {
    pub category: Category,

    pub description: String,

    pub image_name: String,

    pub masterable: bool,

    pub name: String,

    pub tradable: bool,

    #[serde(rename = "type")]
    pub quest_type: String,

    pub unique_name: String,
}

#[tokio::test]
async fn test_quest_query() -> Result<(), Box<dyn std::error::Error>> {
    let _quest = reqwest::get("https://api.warframestat.us/items/second%20dream/")
        .await?
        .json::<Quest>()
        .await?;
    Ok(())
}
