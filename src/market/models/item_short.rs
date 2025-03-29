use serde::Deserialize;

use super::i18n::I18N;

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemShort {
    /// Unique identifier of the item
    pub id: String,
    /// URL-friendly name of the item
    pub slug: String,
    /// Reference to the item in the game's database
    pub game_ref: String,
    /// Tags related to the item
    pub tags: Vec<String>,

    /// Localized text for the item in various languages
    pub i18n: I18N<ItemShortI18N>,

    /// maximum rank the item can achieve
    pub max_rank: Option<i8>,
    /// maximum charges the item can achieve
    pub max_charges: Option<i8>,
    /// flag indicating if the item is vaulted
    pub vaulted: Option<bool>,
    /// flag indicating if the item is bulk tradable
    pub bulk_tradable: Option<bool>,
    /// Ducats value of the item
    pub ducats: Option<i16>,
    /// number of amber stars associated with the item
    pub max_amber_stars: Option<i8>,
    /// number of cyan stars associated with the item
    pub max_cyan_stars: Option<i8>,
    /// base endo value of the item
    pub base_endo: Option<i16>,
    /// multiplier for the endo value
    pub endo_multiplier: Option<f32>,
    /// subtype of the item
    pub subtypes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ItemShortI18N {
    /// Localized name of the item
    pub name: String,
    /// Localized icon of the item
    pub icon: String,
    /// Localized thumbnail of the item
    pub thumb: String,
    /// Optional localized sub-icon of the item
    pub sub_icon: Option<String>,
}

#[cfg(test)]
mod test {
    use super::ItemShort;
    use crate::market::models::ResponseBase;

    #[rstest::rstest]
    fn test_item_short(
        #[files("src/market/models/fixtures/items.json")]
        #[mode = str]
        json: &str,
    ) -> Result<(), serde_json::Error> {
        serde_json::from_str::<ResponseBase<Vec<ItemShort>>>(json)?;

        Ok(())
    }
}
