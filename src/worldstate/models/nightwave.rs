use super::{
    macros::{enum_builder, model_builder},
    RewardType,
};

enum_builder! {
    :"Represents the difficulty of a [Nightwave Challenge](NightwaveChallenge)"
    NightwaveChallengeType;
    :"Easy"
    Easy,
    :"Medium"
    Medium,
    :"Hard"
    Hard,
    :"Unknown"
    Unknown,
}

model_builder! {
    :"A Nightwave challenge"
    NightwaveChallenge,
    rt = obj,
    timed = true;

    :"The ID of this challenge"
    pub id: String,

    :"Whether it is a daily mission or not"
    pub is_daily: bool,

    :"Whether it is an elite mission or not"
    pub is_elite: bool,

    :"The Description of this challenge (what you need to do in order to complete it)"
    description: String = "desc",

    :"The Title of this Challenge"
    pub title: String,

    :"The amount of reputation (aka standing) you get by completing this mission"
    pub reputation: i32,

    :"Whether it is permanent or not"
    pub is_permanent: bool,
}

impl NightwaveChallenge {
    /// Gets the difficulty for this challenge
    pub fn challenge_type(&self) -> NightwaveChallengeType {
        use NightwaveChallengeType::*;
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

model_builder! {
    :"The Current cycle and challenges of Nightwave, a battle-pass-esque rotation and challenge system"
    Nightwave: "/nightwave",
    rt = obj,
    timed = true;

    :"The ID of the Nightwave"
    pub id: String,

    :"The Season of this Nightwave"
    pub season: i32,

    :"The Tag of this Nightwave"
    pub tag: String,

    :"The phase of the nightwave"
    pub phase: i32,

    :"The reward types"
    pub reward_types: Vec<RewardType>,

    :"The active challenges (most likely the weekly rotation)"
    pub active_challenges: Vec<NightwaveChallenge>
}

#[cfg(test)]
mod test {
    use super::Nightwave;
    use crate::worldstate::{client::Client, error::ApiError};

    #[cfg(not(feature = "multilangual"))]
    #[tokio::test]
    async fn test_nightwave() -> Result<(), ApiError> {
        let client = Client::new();

        match client.fetch::<Nightwave>().await {
            Ok(_nightwave) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[cfg(feature = "multilangual")]
    #[tokio::test]
    async fn test_nightwave_ml() -> Result<(), ApiError> {
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_using_lang::<Nightwave>(Language::ZH).await {
            Ok(_nightwave) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
