use super::macros::{
    enum_builder,
    model_builder,
};

enum_builder! {
    :"The State of the Cambion Drift"
    CambionDriftState;

    :"The 'Vome' state"
    Vome = "vome",
    :"The 'Fass' state"
    Fass = "fass"
}

model_builder! {
    :"Cambion Drift info"
    CambionDrift: "/cambionCycle",
    rt = obj,
    timed = true;

    :"The id of the cycle"
    pub id: String,

    :"The state of the cambion drift (vome/fass)"
    pub state: CambionDriftState
}

#[cfg(test)]
mod test {
    use super::CambionDrift;
    use crate::worldstate::{
        client::Client,
        error::ApiError,
    };

    #[cfg(not(feature = "multilangual"))]
    #[tokio::test]
    async fn test_cambiondrift() -> Result<(), ApiError> {
        let client = Client::new();

        match client.fetch::<CambionDrift>().await {
            Ok(_cambiondrift) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[cfg(feature = "multilangual")]
    #[tokio::test]
    async fn test_cambiondrift_ml() -> Result<(), ApiError> {
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_using_lang::<CambionDrift>(Language::ZH).await {
            Ok(_cambiondrift) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
