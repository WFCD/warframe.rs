use warframe_macros::model;

use super::{
    faction::Faction,
    reward::Reward,
    reward_type::RewardType,
};

type DateTime = chrono::DateTime<chrono::Utc>;

/// An defender/attacker of an Invasion
#[model]
pub struct InvasionMember {
    /// The reward of the mission.
    pub reward: Option<Reward>,

    /// The localized faction that houses the node/mission
    pub faction: String,

    /// The faction that houses the node/mission
    pub faction_key: Faction,
}

/// An Invasion
#[model(endpoint = "/invasions", return_style = Array)]
pub struct Invasion {
    /// The time the Invasion began
    pub activation: DateTime,

    /// Whether the Invasion is over
    pub completed: bool,

    /// Percentage of the Invasion's completion
    pub completion: f32,

    /// How many fights have happened
    pub count: i32,

    /// The Invasion's description
    #[serde(rename = "desc")]
    pub description: String,

    /// Short-formatted string estimating the time until the Invasion is closed
    pub eta: String,

    /// The i18n of the node
    pub node: String,

    /// The name of the node
    pub node_key: String,

    /// The amount of runs required to qualify for the reward. (most likely 3)
    pub required_runs: i32,

    /// Whether the fight is against infested enemies
    pub vs_infestation: bool,

    /// The invading faction information
    pub attacker: InvasionMember,

    /// The defending faction information
    pub defender: InvasionMember,

    /// Short-time-formatted duration string of the start of the Invasion
    pub start_string: String,

    /// A list of reward types
    pub reward_types: Vec<RewardType>,
}

#[cfg(test)]
mod test_invasion {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Invasion;
    use crate::worldstate::{
        Queryable,
        fixtures::invasion::{
            invasion,
            invasion_en,
        },
    };

    type R = <Invasion as Queryable>::Return;

    #[rstest]
    fn test(invasion_en: &str) {
        from_str::<R>(invasion_en).unwrap();
    }

    #[rstest]
    fn test_ml(invasion: &str) {
        from_str::<R>(invasion).unwrap();
    }
}
