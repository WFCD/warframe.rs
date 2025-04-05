use serde::Deserialize;

use super::{
    i18n::I18N,
    impl_queryable,
};

impl_queryable!(SisterWeapon, Array, "/sister/weapons");

/// Represents the `/sister/weapons` endpoint
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SisterWeapon {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub req_mastery_rank: u8,
    pub i18n: I18N<SisterWeaponI18N>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SisterWeaponI18N {
    pub name: String,
    pub wiki_link: Option<String>,
    pub icon: String,
    pub thumb: String,
}

#[cfg(test)]
mod test {
    use super::SisterWeapon;
    use crate::market::{
        Queryable,
        models::ResponseBase,
    };

    #[rstest::rstest]
    fn lich_weapon(
        #[files("src/market/models/fixtures/lich_weapon.json")]
        #[mode = str]
        json: &str,
    ) -> Result<(), serde_json::Error> {
        serde_json::from_str::<ResponseBase<<SisterWeapon as Queryable>::Data>>(json)?;

        Ok(())
    }
}
