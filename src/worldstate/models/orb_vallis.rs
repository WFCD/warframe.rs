use warframe_macros::model;

/// Represents the state on Orb Vallis
#[model]
#[serde(rename_all = "lowercase")]
pub enum OrbVallisState {
    /// Warm
    Warm,
    /// Cold
    Cold,
}

/// The current cycle of the Orb Vallis warm/cold cycle
#[model(endpoint = "/vallisCycle", return_style = Object, timed)]
pub struct OrbVallis {
    /// Unique identifier for this object/event/thing
    pub id: String,

    /// The state of the orb vallis (warm/cold)
    pub state: OrbVallisState,
}

#[cfg(test)]
mod test {
    use super::OrbVallis;
    use crate::worldstate::{
        client::Client,
        error::Error,
    };

    #[tokio::test]
    async fn test_orbvallis() -> Result<(), Error> {
        let client = Client::new();

        match client.fetch::<OrbVallis>().await {
            Ok(_orbvallis) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[tokio::test]
    async fn test_orbvallis_ml() -> Result<(), Error> {
        use crate::worldstate::language::Language;

        let client = Client::new();

        match client.fetch_using_lang::<OrbVallis>(Language::ZH).await {
            Ok(_orbvallis) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
