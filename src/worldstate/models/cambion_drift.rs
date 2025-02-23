use warframe_macros::model;

/// The State of the Cambion Drift
#[model]
#[serde(rename_all = "lowercase")]
pub enum CambionDriftState {
    /// The 'Vome' state
    Vome,
    /// The 'Fass' state
    Fass,
}

/// Cambion Drift info
#[model(endpoint = "/cambionCycle", return_style = Object, timed)]
pub struct CambionDrift {
    /// The id of the cycle
    pub id: String,

    /// The state of the cambion drift (vome/fass)
    pub state: CambionDriftState,
}

#[cfg(test)]
mod test {
    use super::CambionDrift;
    use crate::worldstate::{
        client::Client,
        error::Error,
    };

    #[tokio::test]
    async fn test_cambiondrift() -> Result<(), Error> {
        let client = Client::new();

        match client.fetch::<CambionDrift>().await {
            Ok(_cambiondrift) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[tokio::test]
    async fn test_cambiondrift_ml() -> Result<(), Error> {
        use crate::worldstate::language::Language;

        let client = Client::new();

        match client.fetch_using_lang::<CambionDrift>(Language::ZH).await {
            Ok(_cambiondrift) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
