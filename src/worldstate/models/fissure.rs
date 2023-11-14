use super::enemy::Enemy;
use super::macros::{enum_builder, model_builder};
use super::mission_type::MissionType;

enum_builder! {
    Tier;
    Lith,
    Meso,
    Neo,
    Axi,
    Requiem,
}

model_builder! {
    Fissure: "/fissures",
    rt = array,
    timed = true;

    :"The id of the fissure"
    pub id: String,

    :"The i18n of the mission"
    pub mission_type: String,

    :"The type of the mission"
    pub mission_key: MissionType,

    :"The i18n of the node"
    pub node: String,

    :"The name of the node"
    pub node_key: String,

    :"The tier of the relic"
    pub tier: Tier,

    :"The number of the tier (1-5)"
    pub tier_num: u8,

    :"The i18n name of the enemy"
    pub enemy: String,

    :"The type of the enemy"
    pub enemy_key: Enemy,

    :"Whether the fissure is a storm"
    pub is_storm: bool,

    :"Whether the the fissure is hard (Steel Path)"
    pub is_hard: bool
}
