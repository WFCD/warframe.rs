use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub faction_index: i64,

    pub masterable: bool,

    pub mastery_req: i64,

    pub max_enemy_level: i64,

    pub min_enemy_level: i64,

    pub mission_index: i64,

    pub name: String,

    #[serde(rename = "nodeType")]
    pub type_: i64,

    pub system_index: i64,

    pub system_name: String,

    pub tradable: bool,

    #[serde(rename = "type")]
    pub node_type: String,

    pub unique_name: String,
}
