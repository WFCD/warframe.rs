use warframe_macros::model;

use super::{
    Faction,
    Reward,
    Syndicate,
};

/// An Event in Warframe
#[model(endpoint = "/events", return_style = Array, timed)]
pub struct Event {
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
}

#[cfg(test)]
mod test_event {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Event;
    use crate::worldstate::{
        fixtures::event::{
            event,
            event_en,
        },
        models::Queryable,
    };

    type R = <Event as Queryable>::Return;

    #[rstest]
    fn test(event_en: &str) {
        from_str::<R>(event_en).unwrap();
    }

    #[rstest]
    fn test_ml(event: &str) {
        from_str::<R>(event).unwrap();
    }
}
