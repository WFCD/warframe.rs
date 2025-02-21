use super::macros::{
    enum_builder,
    model_builder,
};

enum_builder! {
    :"Represents the state on Orb Vallis"
    OrbVallisState;

    :"Warm"
    Warm = "warm",
    :"Cold"
    Cold = "cold"
}

model_builder! {
    :"The current cycle of the Orb Vallis warm/cold cycle"
    OrbVallis: "/vallisCycle",
    rt = obj,
    timed = true;

    :"Unique identifier for this object/event/thing"
    pub id: String,

    :"The state of the orb vallis (warm/cold)"
    pub state: OrbVallisState
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
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_using_lang::<OrbVallis>(Language::ZH).await {
            Ok(_orbvallis) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
