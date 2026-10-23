use std::collections::HashMap;

use serde_json::Value;
use warframe_macros::model;

use super::{
    faction::Faction,
    reward::Reward,
    syndicate::Syndicate,
    syndicate_mission::SyndicateJob,
};

/// An interim step/milestone in an event
#[model]
pub struct InterimStep {
    /// The goal score for this step
    pub goal: i32,

    /// The reward for reaching this step
    pub reward: Reward,

    /// Message associated with this step
    #[serde(default)]
    pub message: HashMap<String, Value>,
}

/// An Event in Warframe
#[model(endpoint = "/events", return_style = Array, timed)]
pub struct Event {
    /// Unique identifier for this event
    pub id: Option<String>,

    /// Maximum score to complete the event
    pub maximum_score: Option<i32>,

    /// The current score for the event
    pub current_score: Option<i32>,

    /// Interval for the first goal
    pub small_interval: Option<i32>,

    /// Interval for the second intermediate score
    pub large_interval: Option<i32>,

    /// The faction you're up against
    pub faction: Option<Faction>,

    /// The description for the event
    pub description: Option<String>,

    /// Tooltip for the event
    pub tooltip: Option<String>,

    /// Node that the event is taking place on
    pub node: Option<String>,

    /// Nodes that the event is happening concurrently on
    pub concurrent_nodes: Vec<String>,

    /// Node that is being attacked & defended in the event
    pub victim_node: Option<String>,

    /// Localized tag for the event score
    pub score_loc_tag: Option<String>,

    /// The rewards to earn
    pub rewards: Vec<Reward>,

    /// Amount of health remaining for the target
    pub health: Option<f32>,

    /// The associated Syndicate
    pub affiliated_with: Option<Syndicate>,

    /// Active bounties/jobs for this event
    #[serde(default)]
    pub jobs: Vec<SyndicateJob>,

    /// Previous bounties/jobs for this event
    #[serde(default)]
    pub previous_jobs: Vec<SyndicateJob>,

    /// Tag for the event
    pub tag: Option<String>,

    /// Interim milestone steps with their own rewards
    #[serde(default)]
    pub interim_steps: Vec<InterimStep>,

    /// Whether to show the total score at the end of a mission
    #[serde(default)]
    pub show_total_at_end_of_mission: bool,

    /// Whether the event is personal (per-player progress)
    #[serde(default)]
    pub is_personal: bool,

    /// Whether the event is community-driven
    #[serde(default)]
    pub is_community: bool,

    /// Variable name for tracking score
    pub score_var: Option<String>,
}

#[cfg(test)]
mod test_event {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Event;
    use crate::worldstate::Queryable;

    type R = <Event as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/event.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
