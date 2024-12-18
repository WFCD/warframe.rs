use serde::Deserialize;

use super::Category;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Pet {
    pub armor: i64,

    pub category: Category,

    pub description: String,

    pub health: i64,

    pub image_name: String,

    pub masterable: bool,

    pub name: String,

    pub power: i64,

    pub product_category: String,

    pub shield: i64,

    pub stamina: i64,

    pub tradable: bool,

    #[serde(rename = "type")]
    pub pet_type: String,

    pub unique_name: String,
}

#[tokio::test]
async fn test_pet_query() -> Result<(), Box<dyn std::error::Error>> {
    let _pet = reqwest::get("https://api.warframestat.us/items/smeeta%20kavat/")
        .await?
        .json::<Pet>()
        .await?;
    Ok(())
}
