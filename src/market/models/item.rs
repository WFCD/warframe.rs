use serde::Deserialize;

use super::i18n::I18N;

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub struct Item {
    pub id: String,
    pub tags: Vec<String>,
    pub slug: String,
    pub game_ref: String,
    pub tradable: bool,

    pub set_root: Option<bool>,
    #[serde(default)]
    pub set_parts: Vec<String>,
    pub quantity_in_set: Option<i64>,

    pub rarity: Option<String>,
    pub max_rank: Option<u8>,
    pub max_charges: Option<u8>,
    pub bulk_tradable: Option<bool>,
    #[serde(default)]
    pub subtypes: Vec<String>,

    pub max_amber_stars: Option<u8>,
    pub max_cyan_stars: Option<u8>,
    pub base_endo: Option<u16>,
    pub endo_multiplier: Option<f32>,

    pub ducats: Option<u16>,
    pub req_mastery_rank: Option<u8>,
    pub vaulted: Option<bool>,
    pub trading_tax: Option<i64>,

    pub i18n: I18N<ItemI18N>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ItemI18N {
    pub name: String,
    pub description: Option<String>,
    pub wiki_link: Option<String>,
    pub icon: String,
    pub thumb: String,
    pub sub_icon: Option<String>,
}

#[cfg(test)]
mod test {

    use super::Item;
    use crate::market::{
        error::Error,
        models::base::ResponseBase,
    };

    #[rstest::rstest]
    fn test_item(
        #[files("src/market/models/fixtures/item.json")]
        #[mode = str]
        json: &str,
    ) -> Result<(), Error> {
        serde_json::from_str::<ResponseBase<Item>>(json)?;

        Ok(())
    }
}
