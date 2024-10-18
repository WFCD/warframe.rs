//! Warframe type and utils

use {
    super::{Component, Introduced, Polarity},
    serde::{Deserialize, Serialize},
};

/// A Warframe
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Warframe {
    abilities: Vec<Ability>,

    armor: i64,

    aura: String,

    build_price: i64,

    build_quantity: i64,

    build_time: i64,

    category: String,

    color: i64,

    components: Vec<Component>,

    conclave: bool,

    consume_on_build: bool,

    description: String,

    health: i64,

    image_name: String,

    introduced: Introduced,

    // is_prime should be here, but due to alignment I'll remove it
    // since it can easily be evaluated by checking `market_cost` for Option::Some
    market_cost: Option<i64>,

    masterable: bool,

    mastery_req: i64,

    name: String,

    passive_description: String,

    polarities: Vec<Polarity>,

    power: i64,

    product_category: String,

    release_date: String,

    sex: Sex,

    shield: i64,

    skip_build_time_price: i64,

    sprint: f64,

    sprint_speed: f64,

    stamina: f64,

    tradable: bool,

    #[serde(rename = "type")]
    warframe_type: String,

    unique_name: String,

    wikia_thumbnail: String,

    wikia_url: String,
}

/// An ability
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    unique_name: String,

    name: String,

    description: String,

    image_name: String,
}

/// A Warframe's Sex (or gender)
#[derive(Serialize, Deserialize)]
pub enum Sex {
    /// Male
    Male,

    /// Female
    Female,

    /// Non-binary/agender
    #[serde(rename(deserialize = "Non-binary (Pluriform)"))]
    Neutral,
}

#[tokio::test]
async fn test_warframe_query() -> Result<(), Box<dyn std::error::Error>> {
    let _warframe = reqwest::get("https://api.warframestat.us/items/valkyr%20prime/")
        .await?
        .json::<Warframe>()
        .await?;

    Ok(())
}
