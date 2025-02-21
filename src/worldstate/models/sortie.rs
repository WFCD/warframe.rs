use super::{
    macros::model_builder,
    Faction,
};

model_builder! {
    :"A Mission corresponding to a Sortie"
    SortieMission,
    rt = obj,
    timed = false;

    :"The i18n Mission type of this mission"
    pub mission_type: String,

    :"The Modifier of this mission (e.g. Augmented Enemy Armor)"
    pub modifier: String,

    :"The description of the modifier of this mission (e.g. Enemies have Improved/Added armor. Corrosive Projection effects are halved.)"
    pub modifier_description: String,

    :"The i18n of the name"
    pub node: String,
}

model_builder! {
    :"Data about the missions for the current sortie"
    Sortie: "/sortie",
    rt = obj,
    timed = true;

    :"Unique identifier for this object/event/thing"
    pub id: String,

    :"The name of the boss"
    pub boss: String,

    :"The faction you are up against"
    pub faction: Faction,

    :"The 3 missions corresponding to this sortie"
    pub missions: [SortieMission; 3] = "variants",
}

#[cfg(test)]
mod test {
    use super::Sortie;
    use crate::worldstate::{
        client::Client,
        error::Error,
    };

    
    #[tokio::test]
    async fn test_sortie() -> Result<(), Error> {
        let client = Client::new();

        match client.fetch::<Sortie>().await {
            Ok(_sortie) => Ok(()),
            Err(why) => Err(why),
        }
    }

    
    #[tokio::test]
    async fn test_sortie_ml() -> Result<(), Error> {
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_using_lang::<Sortie>(Language::ZH).await {
            Ok(_sortie) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
