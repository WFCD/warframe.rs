use warframe_macros::model;

/// Represents the current state on cetus
#[model]
#[serde(rename_all = "lowercase")]
pub enum CetusState {
    /// Represents Cetus' day state
    Day,
    /// Represents Cetus' night state
    Night,
}

/// The Information about cetus
#[model(endpoint = "/cetusCycle", return_style = Object, timed)]
pub struct Cetus {
    /// The id of the cycle
    pub id: String,

    /// The state of Cetus (day/night)
    pub state: CetusState,
}

#[cfg(test)]
mod test {
    use super::Cetus;
    use crate::worldstate::{
        client::Client,
        error::Error,
        models::{
            CetusState,
            Opposite,
        },
    };

    #[tokio::test]
    async fn test_cetus() -> Result<(), Error> {
        let client = Client::new();

        match client.fetch::<Cetus>().await {
            Ok(_cetus) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[tokio::test]
    async fn test_cetus_ml() -> Result<(), Error> {
        use crate::worldstate::language::Language;

        let client = Client::new();

        match client.fetch_using_lang::<Cetus>(Language::ZH).await {
            Ok(_cetus) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[test]
    fn test_cetus_state_opposite() {
        assert_eq!(CetusState::Day.opposite(), CetusState::Night);
    }
}
