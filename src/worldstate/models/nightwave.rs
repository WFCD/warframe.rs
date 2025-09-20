use warframe_macros::model;

use super::reward_type::RewardType;

/// Represents the difficulty of a [Nightwave Challenge](NightwaveChallenge)
#[model]
pub enum NightwaveChallengeType {
    /// Easy
    Easy,
    /// Medium
    Medium,
    /// Hard
    Hard,
    /// Unknown
    Unknown,
}

/// A Nightwave challenge
#[model(timed)]
pub struct NightwaveChallenge {
    /// The ID of this challenge
    pub id: String,

    /// Whether it is a daily mission or not
    pub is_daily: bool,

    /// Whether it is an elite mission or not
    pub is_elite: bool,

    /// The Description of this challenge (what you need to do in order to complete it)
    #[serde(rename = "desc")]
    pub description: String,

    /// The Title of this Challenge
    pub title: String,

    /// The amount of reputation (aka standing) you get by completing this mission
    pub reputation: i32,

    /// Whether it is permanent or not
    pub is_permanent: bool,
}

impl NightwaveChallenge {
    /// Gets the difficulty for this challenge
    #[must_use]
    pub fn challenge_type(&self) -> NightwaveChallengeType {
        use NightwaveChallengeType::{
            Easy,
            Hard,
            Medium,
            Unknown,
        };
        if self.is_permanent {
            return Unknown;
        }

        match (self.is_daily, self.is_elite) {
            (true, false) => Easy,
            (false, false) => Medium,
            (false, true) => Hard,
            _ => Unknown,
        }
    }
}

/// The Current cycle and challenges of Nightwave, a battle-pass-esque rotation and challenge system
#[model(endpoint = "/nightwave", return_style = Object, timed)]
pub struct Nightwave {
    /// The ID of the Nightwave
    pub id: String,

    /// The Season of this Nightwave
    pub season: i32,

    /// The Tag of this Nightwave
    pub tag: String,

    /// The phase of the nightwave
    pub phase: i32,

    /// The active challenges (most likely the weekly rotation)
    pub active_challenges: Vec<NightwaveChallenge>,
}

#[cfg(test)]
mod test_nightwave {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Nightwave;
    use crate::worldstate::Queryable;

    type R = <Nightwave as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/nightwave.json")]
        #[mode = str]
        nightwave_en: &str,
    ) {
        from_str::<R>(nightwave_en).unwrap();
    }
}
