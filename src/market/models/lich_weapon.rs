use serde::Deserialize;

use super::{
    i18n::I18N,
    impl_queryable,
};

impl_queryable!(LichWeapon, Array, "/lich/weapons");

/// Represents the `/lich/weapons` endpoint
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LichWeapon {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub req_mastery_rank: u8,
    pub i18n: I18N<LichWeaponI18N>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LichWeaponI18N {
    pub name: String,
    pub wiki_link: Option<String>,
    pub icon: String,
    pub thumb: String,
}

#[cfg(test)]
mod test {
    use super::LichWeapon;
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
        serde_json::from_str::<ResponseBase<<LichWeapon as Queryable>::Data>>(json)?;

        Ok(())
    }
}
