use warframe_macros::model;

use crate::worldstate::queryable::Syndicate;

type DateTime = chrono::DateTime<chrono::Utc>;

/// A Syndicate Job (aka Bounty)
#[model]
pub struct SyndicateJob {
    /// Unique identifier for this object/event/thing
    pub id: String,

    /// The Reward Pool of the Bounty
    pub reward_pool: Vec<String>,

    /// The type (or name) of the syndicate job
    #[serde(rename = "type")]
    pub job_type: String,

    /// The level of the Enemies in this job
    pub enemy_levels: Vec<i32>,

    /// The amount of standing you get after completing each stage
    pub standing_stages: Vec<i32>,

    /// The minimum Mastery Rank required for this job
    #[serde(rename = "minMR")]
    pub minimum_mr: i32,

    /// Expiry when this mission expires/disappears
    pub expiry: DateTime,
}

/// All Syndicate Missions (including Cetus, etc.)\nNote that they *may* be empty, in which case
/// they are not valid.
#[model(endpoint =  "/syndicateMissions", return_style = Array, timed)]
pub struct SyndicateMission {
    /// Unique identifier for this object/event/thing
    pub id: String,

    /// The i18n of the syndicate
    pub syndicate: String,

    /// The Syndicate TYPE
    pub syndicate_key: Syndicate,

    /// The Nodes some missions related to the syndicate are on
    pub nodes: Vec<String>,

    /// The jobs this syndicate has to offer
    pub jobs: Vec<SyndicateJob>,
}

#[cfg(test)]
mod test_syndicate_mission {
    use rstest::rstest;
    use serde_json::from_str;

    use super::SyndicateMission;
    use crate::worldstate::Queryable;

    type R = <SyndicateMission as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/syndicate_mission.json")]
        #[mode = str]
        syndicate_mission_en: &str,
    ) {
        from_str::<R>(syndicate_mission_en).unwrap();
    }
}
