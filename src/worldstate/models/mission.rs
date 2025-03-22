use warframe_macros::model;

use super::{
    faction::Faction,
    mission_type::MissionType,
    reward::Reward,
};

/// A mission
#[model]
pub struct Mission {
    /// The reward of this mission
    pub reward: Reward,

    /// The i18n of the node
    pub node: String,

    /// The name of the node
    pub node_key: String,

    /// The i18n faction you are up against
    pub faction: String,

    /// The faction you are up against
    pub faction_key: Faction,

    /// The minimum level of the enemy
    pub min_enemy_level: i32,

    /// The maximum level of the enemy
    pub max_enemy_level: i32,

    /// The maximum wave you can get to
    pub max_wave_num: Option<i32>,

    /// The i18n type of the mission
    pub r#type: String,

    /// The type of the mission
    pub type_key: MissionType,

    /// Whether the mission is a nightmare mission
    pub nightmare: bool,

    /// Whether the mission requires an archwing
    pub archwing_required: bool,

    /// Whether the mission is a sharkwing mission
    pub is_sharkwing: bool,

    /// The enemy spec
    pub enemy_spec: String,

    /// Any level override
    pub level_override: String,

    /// Any additional spawners
    pub advanced_spawners: Vec<String>,

    /// Items required to enter the mission
    pub required_items: Vec<String>,

    /// Whether the required items are consumed
    pub consume_required_items: Option<bool>,

    /// Affectors of this mission
    pub level_auras: Vec<String>,

    /// Description of the mission
    pub description: Option<String>,
}
