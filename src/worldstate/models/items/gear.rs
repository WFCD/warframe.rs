use serde::Deserialize;

use super::Component;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Gear {
    pub build_price: i64,

    pub build_quantity: i64,

    pub build_time: i64,

    pub components: Vec<Component>,

    pub description: String,

    pub image_name: String,

    pub masterable: bool,

    pub name: String,

    pub skip_build_time_price: i64,

    pub tradable: bool,

    pub unique_name: String,
}
