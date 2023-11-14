use super::enemy::Enemy;
use super::mission_type::MissionType;
use crate::{enum_builder, model_builder};

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
    pub id: String,
    pub mission_type: String,
    pub mission_key: MissionType,
    pub node: String,
    pub node_key: String,
    pub tier: Tier,
    pub enemy: String,
    pub enemy_key: Enemy,
    pub is_storm: bool,
    pub is_hard: bool
}
