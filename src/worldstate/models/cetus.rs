use super::macros::{enum_builder, model_builder};

enum_builder! {
    :"Represents the current state on cetus"
    CetusState;

    :"Represents Cetus' day state"
    Day = "day",

    :"Rpresents Cetus' night state"
    Night = "night",
}

model_builder! {
    :"The Information about cetus"
    Cetus: "/cetusCycle",
    rt = obj,
    timed = true;

    :"The id of the cycle"
    pub id: String,

    :"The state of Cetus (day/night)"
    pub state: CetusState,
}

#[cfg(test)]
mod test {
    use super::Cetus;
    use crate::worldstate::{
        client::Client,
        error::ApiError,
        prelude::{CetusState, Opposite},
    };

    #[tokio::test]
    async fn test_cetus() -> Result<(), ApiError> {
        let client = Client::new();

        match client.fetch::<Cetus>().await {
            Ok(_cetus) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[cfg(feature = "multilangual")]
    #[tokio::test]
    async fn test_cetus_ml() -> Result<(), ApiError> {
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_using_lang::<Cetus>(Language::ZH).await {
            Ok(_cetus) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[test]
    fn test_cetus_state_opposite() {
        assert_eq!(CetusState::Day.opposite(), CetusState::Night)
    }
}
