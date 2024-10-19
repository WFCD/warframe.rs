use serde::Deserialize;

use super::Category;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Sigil {
    category: Category,

    description: String,

    image_name: String,

    masterable: bool,

    name: String,

    tradable: bool,

    #[serde(rename = "type")]
    sigil_type: String,

    unique_name: String,
}

#[tokio::test]
async fn test_sigil_query() -> Result<(), Box<dyn std::error::Error>> {
    let _sigil = reqwest::get("https://api.warframestat.us/items/Accord%20Sigil/")
        .await?
        .json::<Sigil>()
        .await?;
    Ok(())
}
