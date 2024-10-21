use std::collections::HashMap;

use chrono::NaiveDate;
use serde::Deserialize;

use super::{
    Category,
    Component,
    Introduced,
    Polarity,
};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Weapon {
    pub accuracy: f64,

    pub attacks: Vec<Attack>,

    pub build_price: i64,

    pub build_quantity: i64,

    pub build_time: i64,

    pub category: Category,

    pub components: Vec<Component>,

    pub consume_on_build: bool,

    pub critical_chance: f64,

    pub critical_multiplier: i64,

    pub damage: HashMap<String, f64>,

    pub damage_per_shot: Vec<f64>,

    pub description: String,

    pub disposition: f64,

    pub fire_rate: f64,

    pub image_name: String,

    pub introduced: Introduced,

    pub is_prime: bool,

    pub magazine_size: i64,

    pub masterable: bool,

    pub mastery_req: i64,

    pub multishot: f64,

    pub name: String,

    pub noise: String,

    pub omega_attenuation: f64,

    pub polarities: Vec<Polarity>,

    pub proc_chance: f64,

    pub product_category: String,

    pub release_date: NaiveDate,

    pub reload_time: f64,

    pub skip_build_time_price: i64,

    pub slot: i64,

    pub tags: Vec<String>,

    pub total_damage: f64,

    pub tradable: bool,

    pub trigger: Trigger,

    #[serde(rename = "type")]
    pub weapon_type: String,

    pub unique_name: String,

    pub vaulted: bool,

    pub wikia_thumbnail: String,

    pub wikia_url: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Attack {
    pub name: String,

    pub speed: i64,

    pub crit_chance: f64,

    pub crit_mult: f64,

    pub status_chance: f64,

    pub shot_type: ShotType,

    pub shot_speed: Option<f64>,

    pub flight: Option<i64>,

    pub damage: HashMap<String, f64>,

    pub falloff: Option<Falloff>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Falloff {
    pub start: i64,

    pub end: i64,

    pub reduction: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum Trigger {
    Active,
    Auto,
    #[serde(rename = "Auto Burst")]
    AutoBurst,
    Burst,
    Charge,
    Duplex,
    Held,
    Melee,
    Semi,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum ShotType {
    Continuous,
    #[serde(rename = "Hit-Scan")]
    HitScan,
    Projectile,
    AoE,
}

#[tokio::test]
async fn test_weapon_query() -> Result<(), Box<dyn std::error::Error>> {
    let _weapon = reqwest::get("https://api.warframestat.us/items/acceltra%20prime/")
        .await?
        .json::<Weapon>()
        .await?;
    Ok(())
}
