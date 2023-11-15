use super::macros::model_builder;
use super::{Enemy, MissionType, RewardType};

model_builder! {
    CountedItem,
    rt = array;

    pub count: i32,
    pub r#type: String
}

model_builder! {
    :"The reward of this event"
    AlertMissionReward,
    rt = obj;

    :"Items that have a quantity attached"
    pub counted_items: Vec<CountedItem>,

    :"Thumbnail URL"
    pub thumbnail: String,

    :"RGB value as an int assigned to this reward"
    pub color: i32,

    :"Amount of credits awarded"
    pub credits: i32,

    :"string representation of the reward"
    pub as_string: String,

    :"Items' names possible to be won"
    pub items: Vec<String>,

    :"formatted string describing all included items"
    pub item_string: String,
}

model_builder! {
    :"A mission"
    AlertMission,
    rt = obj;

    :"The reward of this mission"
    pub reward: AlertMissionReward,

    :"The i18n of the node"
    pub node: String,

    :"The name of the node"
    pub node_key: String,

    :"The i18n faction you are up against"
    pub faction: Enemy,

    :"The faction you are up against"
    pub faction_key: Enemy,

    :"The minimum level of the enemy"
    pub min_enemy_level: i32,

    :"The maximum level of the enemy"
    pub max_enemy_level: i32,

    pub max_wave_num: i32,

    :"The i18n type of the mission"
    pub r#type: String,

    :"The type of the mission"
    pub type_key: MissionType,

    :"Whether the mission is a nightmare mission"
    pub nightmare: bool,

    :"Whether the mission requires an archwing"
    pub archwing_required: bool,

    :"Whether the mission is a sharkwing mission"
    pub is_sharkwing: bool,

    pub enemy_spec: String,

    pub level_override: String,

    pub advanced_spawners: Vec<String>,

    :"Items required to enter the mission"
    pub required_items: Vec<String>,

    :"Whether the required items are consumed"
    pub consume_required_items: Option<bool>,

    :"Affectors of this mission"
    pub level_auras: Vec<String>,

    :"Description of the mission"
    pub description: String
}

model_builder! {
    :"An alert in Warframe"
    Alert: "/alerts",
    rt = array;

    :"ID of this event"
    pub id: String,

    :"The mission associated with the alert"
    pub mission: AlertMission,

    :"The reward type of the alert"
    pub reward_type: Option<RewardType>
}
