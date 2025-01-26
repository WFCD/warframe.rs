use std::collections::HashMap;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Stats {
    /// ciphers_failed
    pub ciphers_failed: u32,

    /// ciphers_solved
    pub ciphers_solved: u32,

    /// cipher_time
    pub cipher_time: f64,

    /// capture_event_score
    pub capture_event_score: u32,

    /// deaths
    pub deaths: u32,

    /// rating
    pub rating: i32,

    #[serde(default)]
    /// weapons
    pub weapons: Vec<Weapon>,

    #[serde(default)]
    /// enemies
    pub enemies: Vec<Enemy>,

    /// heal_count
    pub heal_count: u32,

    /// income
    pub income: u64,

    /// melee_kills
    pub melee_kills: u32,

    /// missions_dumped
    pub missions_dumped: u32,

    /// missions_failed
    pub missions_failed: u32,

    /// missions_interrupted
    pub missions_interrupted: u32,

    /// missions_quit
    pub missions_quit: u32,

    /// missions_completed
    pub missions_completed: u32,

    #[serde(default)]
    /// missions
    pub missions: Vec<Mission>,
    
    /// time_played_sec
    pub time_played_sec: f64,
    
    /// pickup_count
    pub pickup_count: u32,
    
    /// player_level
    pub player_level: u8,
    
    /// rank
    pub rank: u8,
    
    /// revive_count
    pub revive_count: u32,
    
    /// sabotage_event_score
    pub sabotage_event_score: Option<u32>,
    
    /// survival_event_score
    pub survival_event_score: Option<u32>,

    #[serde(default)]
    /// abilities
    pub abilities: Vec<Ability>,

    /// infested_event_score
    pub infested_event_score: Option<u32>,

    #[serde(default)]
    /// scans
    pub scans: Vec<Scan>,
    
    /// dojo_obstacle_score
    pub dojo_obstacle_score: Option<u32>,
    
    #[serde(default, rename = "PVP")]
    /// pvp
    pub pvp: Vec<Pvp>,
    
    /// fomorian_event_score
    pub fomorian_event_score: Option<u32>,
    
    /// zephyr_score
    pub zephyr_score: Option<u32>,
    
    /// sentinel_game_score
    pub sentinel_game_score: Option<u32>,
    
    /// project_sinister_event_score
    pub project_sinister_event_score: Option<u32>,
    
    /// pvp_games_pending_mask
    pub pvp_games_pending_mask: Option<u32>,
    
    /// colonist_rescue_event_score_max
    pub colonist_rescue_event_score_max: Option<u32>,
    
    /// ambulas_event_score_max
    pub ambulas_event_score_max: Option<u32>,
    
    #[serde(default, deserialize_with = "deserialize_high_scores")]
    /// races
    pub races: HashMap<String, u32>,
    
    /// halloween_19_score_max
    pub halloween_19_score_max: Option<u32>,
    
    /// flotilla_event_score
    pub flotilla_event_score: Option<u32>,
    
    /// flotilla_ground_badges_tier_1
    pub flotilla_ground_badges_tier_1: Option<u32>,
    
    /// flotilla_ground_badges_tier_2
    pub flotilla_ground_badges_tier_2: Option<u32>,
    
    /// flotilla_ground_badges_tier_3
    pub flotilla_ground_badges_tier_3: Option<u32>,
    
    /// flotilla_space_badges_tier_1
    pub flotilla_space_badges_tier_1: Option<u32>,
    
    /// flotilla_space_badges_tier_2
    pub flotilla_space_badges_tier_2: Option<u32>,
    
    /// flotilla_space_badges_tier_3
    pub flotilla_space_badges_tier_3: Option<u32>,
    
    /// mech_survival_score_max
    pub mech_survival_score_max: Option<u32>,
    
    /// caliber_chicks_score
    pub caliber_chicks_score: Option<u32>,
    
    /// guild_name
    pub guild_name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Weapon {
    #[serde(rename = "type")]
    /// type
    pub unique_name: String,

    /// xp
    pub xp: Option<u64>,

    #[serde(default)]
    /// equip_time
    pub equip_time: f64,

    /// headshots
    pub headshots: Option<u32>,

    /// kills
    pub kills: Option<u32>,

    /// assists
    pub assists: Option<u32>,

    /// hits
    pub hits: Option<u32>,

    /// fired
    pub fired: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Enemy {
    #[serde(rename = "type")]
    /// type
    pub unique_name: String,

    /// executions
    pub executions: Option<u32>,

    /// headshots
    pub headshots: Option<u32>,

    /// kills
    pub kills: Option<u32>,

    /// assists
    pub assists: Option<u32>,

    /// deaths
    pub deaths: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mission {
    #[serde(rename = "type")]
    /// type
    pub unique_name: String,

    /// high_score
    pub high_score: u32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    #[serde(rename = "type")]
    /// type
    pub unique_name: String,

    /// used
    pub used: u32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Scan {
    #[serde(rename = "type")]
    /// type
    pub unique_name: String,

    /// scans
    pub scans: u32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pvp {
    #[serde(rename = "type")]
    /// type
    pub unique_name: String,
    
    /// suit_kills
    pub suit_kills: Option<u32>,

    /// suit_deaths
    pub suit_deaths: Option<u32>,
    
    /// weapon_kills
    pub weapon_kills: Option<u32>,
}

fn deserialize_high_scores<'de, D>(deserializer: D) -> Result<HashMap<String, u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;
    let mut map = HashMap::new();

    if let Value::Object(obj) = v {
        for (key, value) in obj {
            if let Some(high_score) = value.get("highScore").and_then(|v| v.as_u64()) {
                map.insert(key.clone(), high_score as u32);
            }
        }
    }

    Ok(map)
}
