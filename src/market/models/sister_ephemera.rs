use serde::Deserialize;

use super::{
    i18n::I18N,
    impl_queryable,
};

impl_queryable!(SisterEphemera, Array, "/sister/ephemeras");

/// Represents the `/sister/ephemeras` endpoint
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SisterEphemera {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub animation: String,
    pub element: String,
    pub i18n: I18N<SisterEphemeraI18N>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SisterEphemeraI18N {
    pub name: String,
    pub icon: String,
    pub thumb: String,
}

#[cfg(test)]
mod test {
    use super::SisterEphemera;
    use crate::market::{
        Queryable,
        models::ResponseBase,
    };

    #[rstest::rstest]
    fn lich_ephemera(
        #[files("src/market/models/fixtures/lich_ephemera.json")]
        #[mode = str]
        json: &str,
    ) -> Result<(), serde_json::Error> {
        serde_json::from_str::<ResponseBase<<SisterEphemera as Queryable>::Data>>(json)?;

        Ok(())
    }
}
