use std::collections::HashMap;

use chrono::NaiveDate;
use serde::Deserialize;

use super::{
    Category,
    Component,
    Introduced,
    Polarity,
};
use crate::worldstate::models::damage_type::DamageType;

fn as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

fn as_f64_option<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;
    if let Some(s) = s {
        s.parse().map(Some).map_err(serde::de::Error::custom)
    } else {
        Ok(None)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Weapon {
    Rifle(RangedWeapon),

    Shotgun(RangedWeapon),

    Pistol(RangedWeapon),

    #[serde(rename = "Arch-Gun")]
    ArchGun(RangedWeapon),

    Melee(MeleeWeapon),

    #[serde(rename = "Arch-Melee")]
    ArchMelee(MeleeWeapon),

    #[serde(rename = "Companion Weapon")]
    CompanionWeapon(RangedWeapon),
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RangedWeapon {
    pub accuracy: f64,

    pub attacks: Vec<RangedAttack>,

    pub build_price: f64,

    pub build_quantity: i64,

    pub build_time: i64,

    pub category: Category,

    pub components: Vec<Component>,

    pub consume_on_build: bool,

    pub critical_chance: f64,

    pub critical_multiplier: f64,

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

    pub unique_name: String,

    /// This will be [Some], if [`RangedWeapon::is_prime`] is true
    pub vaulted: Option<bool>,

    pub wikia_thumbnail: String,

    pub wikia_url: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct RangedAttack {
    pub name: String,

    pub duration: Option<f64>,

    pub charge_time: Option<f64>,

    pub channeling: Option<f64>,

    pub speed: f64,

    pub crit_chance: f64,

    pub crit_mult: f64,

    pub status_chance: f64,

    pub shot_type: ShotType,

    pub shot_speed: Option<f64>,

    pub flight: Option<f64>,

    pub damage: HashMap<DamageType, f64>,

    pub falloff: Option<Falloff>,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MeleeWeapon {
    pub attacks: Vec<MeleeAttack>,

    pub build_price: f64,

    pub build_quantity: i64,

    pub build_time: i64,

    pub category: Category,

    pub components: Vec<Component>,

    pub consume_on_build: bool,

    pub critical_chance: f64,

    pub critical_multiplier: f64,

    pub damage: HashMap<DamageType, f64>,

    pub damage_per_shot: Vec<f64>,

    pub description: String,

    pub disposition: f64,

    #[serde(rename = "fireRate")]
    pub attack_speed: f64,

    pub image_name: String,

    pub introduced: Introduced,

    pub is_prime: bool,

    pub masterable: bool,

    pub mastery_req: i64,

    pub name: String,

    pub omega_attenuation: f64,

    pub polarities: Vec<Polarity>,

    pub proc_chance: f64,

    pub product_category: String,

    pub release_date: NaiveDate,

    pub skip_build_time_price: i64,

    pub slot: i64,

    pub tags: Vec<String>,

    pub total_damage: f64,

    pub tradable: bool,

    pub unique_name: String,

    /// This will be [Some], if [`MeleeWeapon::is_prime`] is true
    pub vaulted: Option<bool>,

    pub wikia_thumbnail: String,

    pub wikia_url: String,

    pub stance_polarity: Polarity,

    pub blocking_angle: f64,

    pub combo_duration: f64,

    pub follow_through: f64,

    pub range: f64,

    pub slam_attack: f64,

    pub slam_radial_damage: f64,

    pub slam_radius: f64,

    pub slide_attack: f64,

    pub heavy_attack_damage: f64,

    pub heavy_slam_attack: f64,

    pub heavy_slam_radial_damage: f64,

    pub heavy_slam_radius: f64,

    pub wind_up: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct MeleeAttack {
    pub name: String,

    pub duration: Option<f64>,

    #[serde(rename = "chargeTime")]
    pub wind_up: Option<f64>,

    pub speed: f64,

    pub crit_chance: f64,

    pub crit_mult: f64,

    pub status_chance: f64,

    pub shot_speed: Option<f64>,

    pub flight: Option<f64>,

    pub damage: HashMap<DamageType, f64>,

    pub falloff: Option<Falloff>,

    #[serde(default, deserialize_with = "as_f64_option")]
    pub slide: Option<f64>,

    pub slam: Option<SlamAttack>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct RadialAttack {
    #[serde(deserialize_with = "as_f64")]
    pub damage: f64,

    // #[serde(deserialize_with = "as_f64")]
    pub radius: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SlamAttack {
    #[serde(deserialize_with = "as_f64")]
    pub damage: f64,
    pub radial: RadialAttack,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Falloff {
    pub start: f64,

    pub end: f64,

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
    Thrown,
}

#[cfg(test)]
mod test_weapon {
    use rstest::rstest;
    use serde_json::from_str;

    use crate::worldstate::models::items::weapon::Weapon;

    #[rstest]
    fn test_weapon_query_primary(
        #[files("src/worldstate/models/fixtures/weapon_ranged.json")]
        #[mode = str]
        ranged_weapon_en: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let weapon = from_str::<Weapon>(ranged_weapon_en)?;

        assert!(matches!(weapon, Weapon::Rifle(_)));

        Ok(())
    }

    #[rstest]
    fn test_weapon_query_melee(
        #[files("src/worldstate/models/fixtures/weapon_melee.json")]
        #[mode = str]
        melee_weapon_en: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let weapon = from_str::<Weapon>(melee_weapon_en)?;

        assert!(matches!(weapon, Weapon::Melee(_)));

        Ok(())
    }
}
