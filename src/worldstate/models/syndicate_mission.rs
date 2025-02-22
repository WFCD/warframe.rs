use warframe_macros::model;

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
    pub syndicate_key: String,

    /// The Nodes some missions related to the syndicate are on
    pub nodes: Vec<String>,

    /// The jobs this syndicate has to offer
    pub jobs: Vec<SyndicateJob>,
}

#[cfg(test)]
mod test {
    use super::SyndicateMission;
    use crate::worldstate::{
        client::Client,
        error::Error,
    };

    #[tokio::test]
    async fn test_syndicate() -> Result<(), Error> {
        let client = Client::new();

        match client.fetch::<SyndicateMission>().await {
            Ok(_syndicates) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[tokio::test]
    async fn test_syndicate_ml() -> Result<(), Error> {
        use crate::worldstate::language::Language;

        let client = Client::new();

        match client
            .fetch_using_lang::<SyndicateMission>(Language::ZH)
            .await
        {
            Ok(_syndicates) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
