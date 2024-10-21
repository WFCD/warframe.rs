use serde::Deserialize;

use super::{
    Category,
    Drop,
};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    category: Category,

    description: String,

    drops: Vec<Drop>,

    image_name: String,

    item_count: i64,

    masterable: bool,

    name: String,

    parents: Vec<String>,

    tradable: bool,

    #[serde(rename = "type")]
    resource_type: String,

    unique_name: String,
}

#[tokio::test]
async fn test_resource_query() -> Result<(), Box<dyn std::error::Error>> {
    let _resource = reqwest::get("https://api.warframestat.us/items/nano%20spores/")
        .await?
        .json::<Resource>()
        .await?;
    Ok(())
}
