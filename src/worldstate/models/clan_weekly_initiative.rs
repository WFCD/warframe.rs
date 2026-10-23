use warframe_macros::model;

/// A reward tier in the clan weekly initiative
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClanWeeklyReward {
    /// Whether the reward has been claimed
    pub reward_claimed: bool,

    /// The point threshold to earn this reward
    pub point_threshold: i32,

    /// The number of items rewarded
    pub item_count: i32,

    /// The unique name of the reward
    pub reward: String,
}

/// The clan weekly initiative with bonus region and rewards
#[model(endpoint = "/clanWeeklyInitiative", return_style = Object, timed)]
pub struct ClanWeeklyInitiative {
    /// The current week number
    pub week: i32,

    /// The i18n name of the bonus region
    pub bonus_region: String,

    /// The unique name of the bonus region
    pub region_unique_name: String,

    // Note: the API has a typo "rewwards" (double w)
    /// The reward tiers
    #[serde(rename = "rewwards")]
    pub rewards: Vec<ClanWeeklyReward>,
}

#[cfg(test)]
mod test_clan_weekly_initiative {
    use rstest::rstest;
    use serde_json::from_str;

    use super::ClanWeeklyInitiative;
    use crate::worldstate::Queryable;

    type R = <ClanWeeklyInitiative as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/clan_weekly_initiative.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
