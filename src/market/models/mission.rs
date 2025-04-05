use serde::Deserialize;

use super::{
    i18n::I18N,
    impl_queryable,
};

impl_queryable!(Mission, Array, "/missions");

/// Represents the `/missions` endpoint
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mission {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub i18n: I18N<MissionI18N>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MissionI18N {
    pub name: String,
    pub icon: Option<String>,
    pub thumb: Option<String>,
}

#[cfg(test)]
mod test {
    use super::Mission;
    use crate::market::{
        Queryable,
        models::ResponseBase,
    };

    #[rstest::rstest]
    fn mission(
        #[files("src/market/models/fixtures/mission.json")]
        #[mode = str]
        json: &str,
    ) -> Result<(), serde_json::Error> {
        serde_json::from_str::<ResponseBase<<Mission as Queryable>::Data>>(json)?;

        Ok(())
    }
}
