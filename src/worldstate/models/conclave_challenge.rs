use warframe_macros::model;

/// The category of a Conclave challenge
#[derive(
    Debug, PartialEq, PartialOrd, Clone, Eq, Hash, derive_more::Display, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ConclaveChallengeCategory {
    /// Daily challenge
    Day,
    /// Daily root challenge
    DailyRoot,
    /// Weekly challenge
    Week,
    /// Weekly root challenge
    WeekRoot,
    /// Mode affector challenge
    ModeAffector,
    /// An unrecognized category
    #[serde(untagged)]
    Unknown(String),
}

/// A Conclave challenge
#[model(endpoint = "/conclaveChallenges", return_style = Array, timed)]
pub struct ConclaveChallenge {
    /// Unique identifier for this challenge
    pub id: String,
    /// The number of completions required
    pub amount: i32,
    /// The Conclave game mode this challenge applies to
    pub mode: String,
    /// The i18n category of this challenge
    pub category: String,
    /// The category key
    pub category_key: ConclaveChallengeCategory,
    /// A description of the challenge objective
    pub description: String,
    /// The title of the challenge
    pub title: String,
    /// The standing reward for completing this challenge
    pub standing: i32,
    /// Whether this is a daily challenge
    pub daily: bool,
    /// Whether this is a root challenge
    pub root_challenge: bool,
}

#[cfg(test)]
mod test_conclave_challenge {
    use rstest::rstest;
    use serde_json::from_str;

    use super::ConclaveChallenge;
    use crate::worldstate::Queryable;

    type R = <ConclaveChallenge as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/conclave_challenge.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
