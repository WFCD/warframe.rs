use super::macros::model_builder;

type DateTime = chrono::DateTime<chrono::Utc>;

model_builder! {
    :"A Syndicate Job (aka Bounty)"
    SyndicateJob,
    rt = obj,
    timed = false;

    :"Unique identifier for this object/event/thing"
    pub id: String,

    :"The Reward Pool of the Bounty"
    pub reward_pool: Vec<String>,

    :"The type (or name) of the syndicate job"
    pub job_type: String = "type",

    :"The level of the Enemies in this job"
    pub enemy_levels: Vec<i32>,

    :"The amount of standing you get after completing each stage"
    pub standing_stages: Vec<i32>,

    :"The minimum Mastery Rank required for this job"
    pub minimum_mr: i32 = "minMR",

    :"Expiry when this mission expires/disappears"
    pub expiry: DateTime
}

model_builder! {
    :"All Syndicate Missions (including Cetus, etc.)\nNote that they *may* be empty, in which case they are not valid."
    SyndicateMission: "/syndicateMissions",
    rt = array,
    timed = true;

    :"Unique identifier for this object/event/thing"
    pub id: String,

    :"The i18n of the syndicate"
    pub syndicate: String,

    :"The Syndicate TYPE"
    pub syndicate_key: String,

    :"The Nodes some missions related to the syndicate are on"
    pub nodes: Vec<String>,

    :"The jobs this syndicate has to offer"
    pub jobs: Vec<SyndicateJob>
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
        use crate::worldstate::prelude::Language;

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
