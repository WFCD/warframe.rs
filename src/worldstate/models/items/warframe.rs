//! Warframe type and utils

use serde::Deserialize;

use super::{
    Component,
    Introduced,
    Polarity,
};

#[allow(clippy::struct_excessive_bools)]
/// A Warframe
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Warframe {
    pub abilities: Vec<Ability>,

    pub armor: i64,

    pub aura: Option<String>,

    pub build_price: i64,

    pub build_quantity: i64,

    pub build_time: i64,

    pub color: i64,

    pub components: Vec<Component>,

    pub conclave: bool,

    pub consume_on_build: bool,

    pub description: String,

    pub health: i64,

    pub image_name: String,

    pub introduced: Introduced,

    pub is_prime: bool,

    pub market_cost: Option<i64>,

    pub masterable: bool,

    pub mastery_req: i64,

    pub name: String,

    pub passive_description: String,

    pub polarities: Vec<Polarity>,

    pub power: i64,

    pub product_category: String,

    pub release_date: String,

    pub sex: Sex,

    pub shield: i64,

    pub skip_build_time_price: i64,

    pub sprint: f64,

    pub sprint_speed: f64,

    pub stamina: f64,

    pub tradable: bool,

    #[serde(rename = "type")]
    pub warframe_type: String,

    pub unique_name: String,

    pub wikia_thumbnail: Option<String>,

    pub wikia_url: Option<String>,
}

/// An ability
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    pub unique_name: String,

    pub name: String,

    pub description: String,

    pub image_name: String,
}

/// A Warframe's Sex (or gender)
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum Sex {
    /// Male
    Male,

    /// Female
    Female,

    /// Non-binary/agender
    #[serde(rename(deserialize = "Non-binary (Pluriform)"))]
    Neutral,
}
