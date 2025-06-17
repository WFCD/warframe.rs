//! Archwing type and utils

use serde::Deserialize;

use super::{
    Component,
    warframe::Ability,
};

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Archwing {
    pub abilities: Vec<Ability>,

    pub armor: i64,

    pub build_price: i64,

    pub build_quantity: i64,

    pub build_time: i64,

    pub components: Vec<Component>,

    pub consume_on_build: bool,

    pub description: String,

    pub health: i64,

    pub image_name: String,

    pub is_prime: bool,

    pub masterable: bool,

    pub mastery_req: i64,

    pub name: String,

    pub power: i64,

    pub product_category: String,

    pub shield: i64,

    pub skip_build_time_price: i64,

    pub sprint_speed: f64,

    pub stamina: f64,

    pub tradable: bool,

    pub unique_name: String,
}
