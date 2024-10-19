//! Arcane type and utils

use serde::Deserialize;

use super::{
    Category,
    Drop,
    LevelStat,
    Rarity,
};

/// An Arcane
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Arcane {
    pub category: Category,

    pub drops: Vec<Drop>,

    pub image_name: String,

    pub level_stats: Vec<LevelStat>,

    pub masterable: bool,

    pub name: String,

    pub rarity: Rarity,

    pub tradable: bool,

    pub unique_name: String,
}

#[tokio::test]
async fn test_arcane_query() -> Result<(), Box<dyn std::error::Error>> {
    let _arcane = reqwest::get("https://api.warframestat.us/items/arcane%20energize/")
        .await?
        .json::<Arcane>()
        .await?;
    Ok(())
}
