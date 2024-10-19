use serde::Deserialize;

use super::Category;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Misc {
    pub category: Category,

    pub description: String,

    pub image_name: String,

    pub masterable: bool,

    pub name: String,

    pub tradable: bool,

    #[serde(rename = "type")]
    pub misc_type: String,

    pub unique_name: String,
}

#[tokio::test]
async fn test_misc_query() -> Result<(), Box<dyn std::error::Error>> {
    let _misc = reqwest::get("https://api.warframestat.us/items/Amethyst%20Nexifera%20Tag/")
        .await?
        .json::<Misc>()
        .await?;
    Ok(())
}
