use super::macros::model_builder;
use super::{Faction, Reward, RewardType};

type DateTime = chrono::DateTime<chrono::Utc>;

model_builder! {
    InvasionMember,
    rt = obj,
    timed = false;

    :"The reward of the mission."
    pub reward: Reward,

    :"The localized faction that houses the node/mission"
    pub faction: String,

    :"The faction that houses the node/mission"
    pub faction_key: Faction,
}

model_builder! {
    Invasion: "/invasions",
    rt = array,
    timed = false;

    :"The time the Invasion began"
    pub activation: DateTime,

    :"Whether the Invasion is over"
    pub completed: bool,

    :"Percantage of the Invasion's completion"
    pub completion: f32,

    :"How many fights have happened"
    pub count: i32,

    :"The Invasion's description"
    pub description: String = "desc",

    :"Short-formatted string estimating the time until the Invasion is closed"
    pub eta: String,

    :"The i18n of the node"
    pub node: String,

    :"The name of the node"
    pub node_key: String,

    :"The amount of runs required to qualify for the reward. (most likely 3)"
    pub required_runs: i32,

    :"Whether the fight is against infested enemies"
    pub vs_infestation: bool,

    :"The invading faction information"
    pub attacker: InvasionMember,

    :"The defending faction information"
    pub defender: InvasionMember,

    :"Short-time-formatted duration string of the start of the Invasion"
    pub start_string: String,

    :"A list of reward types"
    pub reward_types: Vec<RewardType>,
}

#[cfg(test)]
mod test {
    use super::Invasion;
    use crate::worldstate::{client::Client, error::ApiError};

    #[cfg(not(feature = "multilangual"))]
    #[tokio::test]
    async fn test_invasion() -> Result<(), ApiError> {
        let client = Client::new();

        match client.fetch_arr::<Invasion>().await {
            Ok(_invasions) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[cfg(feature = "multilangual")]
    #[tokio::test]
    async fn test_invasion_ml() -> Result<(), ApiError> {
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_arr_using_lang::<Invasion>(Language::ZH).await {
            Ok(_invasions) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
