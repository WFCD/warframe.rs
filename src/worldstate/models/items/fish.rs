use serde::Deserialize;

use super::{
    Category,
    Drop,
};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Fish {
    pub category: Category,

    pub description: String,

    pub drops: Vec<Drop>,

    pub image_name: String,

    pub name: String,

    pub tradable: bool,

    pub unique_name: String,
}

#[tokio::test]
async fn test_fish_query() -> Result<(), Box<dyn std::error::Error>> {
    let _fish = reqwest::get("https://api.warframestat.us/items/mawfish/")
        .await?
        .json::<Fish>()
        .await?;
    Ok(())
}
