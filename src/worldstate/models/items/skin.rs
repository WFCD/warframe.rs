use serde::Deserialize;

use super::Category;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Skin {
    pub category: Category,

    pub description: String,

    pub image_name: String,

    pub masterable: bool,

    pub name: String,

    pub tradable: bool,

    #[serde(rename = "type")]
    pub skin_type: String,

    pub unique_name: String,
}

#[tokio::test]
async fn test_skin_query() -> Result<(), Box<dyn std::error::Error>> {
    let _skin = reqwest::get("https://api.warframestat.us/items/Academe%20Ink/")
        .await?
        .json::<Skin>()
        .await?;
    Ok(())
}
