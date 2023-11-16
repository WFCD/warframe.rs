use super::macros::model_builder;
use super::{Mission, RewardType};

model_builder! {
    :"An alert in Warframe"
    Alert: "/alerts",
    rt = array;

    :"ID of this event"
    pub id: String,

    :"The mission associated with the alert"
    pub mission: Mission,

    :"The reward type of the alert"
    pub reward_type: Option<RewardType>
}
