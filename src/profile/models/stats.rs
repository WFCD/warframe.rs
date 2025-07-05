use std::collections::HashMap;

use serde::{
    Deserialize,
    Deserializer,
};
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Stats {
    pub ciphers_failed: u32,

    pub ciphers_solved: u32,

    pub cipher_time: f64,

    pub capture_event_score: u32,

    pub deaths: u32,

    pub rating: i32,

    #[serde(default)]
    pub weapons: Vec<Weapon>,

    #[serde(default)]
    pub enemies: Vec<Enemy>,

    pub heal_count: u32,

    pub income: u64,

    pub melee_kills: u32,

    pub missions_dumped: u32,

    pub missions_failed: u32,

    pub missions_interrupted: u32,

    pub missions_quit: u32,

    pub missions_completed: u32,

    #[serde(default)]
    pub missions: Vec<Mission>,

    pub time_played_sec: f64,

    pub pickup_count: u32,

    pub player_level: u8,

    pub rank: u8,

    pub revive_count: u32,

    pub sabotage_event_score: Option<u32>,

    pub survival_event_score: Option<u32>,

    #[serde(default)]
    pub abilities: Vec<Ability>,

    pub infested_event_score: Option<u32>,

    #[serde(default)]
    pub scans: Vec<Scan>,

    pub dojo_obstacle_score: Option<u32>,

    #[serde(default, rename = "PVP")]
    pub pvp: Vec<Pvp>,

    pub fomorian_event_score: Option<u32>,

    pub zephyr_score: Option<u32>,

    pub sentinel_game_score: Option<u32>,

    pub project_sinister_event_score: Option<u32>,

    pub pvp_games_pending_mask: Option<u32>,

    pub colonist_rescue_event_score_max: Option<u32>,

    pub ambulas_event_score_max: Option<u32>,

    #[serde(default, deserialize_with = "deserialize_high_scores")]
    pub races: HashMap<String, u32>,

    pub halloween_19_score_max: Option<u32>,

    pub flotilla_event_score: Option<u32>,

    pub flotilla_ground_badges_tier_1: Option<u32>,

    pub flotilla_ground_badges_tier_2: Option<u32>,

    pub flotilla_ground_badges_tier_3: Option<u32>,

    pub flotilla_space_badges_tier_1: Option<u32>,

    pub flotilla_space_badges_tier_2: Option<u32>,

    pub flotilla_space_badges_tier_3: Option<u32>,

    pub mech_survival_score_max: Option<u32>,

    pub caliber_chicks_score: Option<u32>,

    pub guild_name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Weapon {
    #[serde(rename = "type")]
    pub unique_name: String,

    pub xp: Option<u64>,

    #[serde(default)]
    pub equip_time: f64,

    pub headshots: Option<u32>,

    pub kills: Option<u32>,

    pub assists: Option<u32>,

    pub hits: Option<u32>,

    pub fired: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Enemy {
    #[serde(rename = "type")]
    pub unique_name: String,

    pub executions: Option<u32>,

    pub headshots: Option<u32>,

    pub kills: Option<u32>,

    pub assists: Option<u32>,

    pub deaths: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mission {
    #[serde(rename = "type")]
    pub unique_name: String,

    pub high_score: u32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    #[serde(rename = "type")]
    pub unique_name: String,

    pub used: u32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Scan {
    #[serde(rename = "type")]
    pub unique_name: String,

    pub scans: u32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pvp {
    #[serde(rename = "type")]
    pub unique_name: String,

    pub suit_kills: Option<u32>,

    pub suit_deaths: Option<u32>,

    pub weapon_kills: Option<u32>,
}

#[allow(clippy::cast_possible_truncation)]
fn deserialize_high_scores<'de, D>(deserializer: D) -> Result<HashMap<String, u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;
    let mut map = HashMap::new();

    if let Value::Object(obj) = v {
        for (key, value) in obj {
            if let Some(high_score) = value.get("highScore").and_then(Value::as_u64) {
                map.insert(key.clone(), high_score as u32);
            }
        }
    }

    Ok(map)
}
