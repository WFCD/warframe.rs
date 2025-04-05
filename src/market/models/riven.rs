use serde::Deserialize;

use super::{
    i18n::I18N,
    impl_queryable,
    riven_group::RivenGroup,
    riven_type::RivenType,
};

impl_queryable!(Riven, Array, "/riven/weapons");

/// Represents the `/riven/weapons` endpoint
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_field_names)]
pub struct Riven {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub group: Option<RivenGroup>,
    pub riven_type: Option<RivenType>,
    pub disposition: f64,
    pub req_mastery_rank: u8,
    pub i18n: I18N<RivenI18N>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RivenI18N {
    item_name: Option<String>,
    wiki_link: Option<String>,
    icon: String,
    thumb: String,
}

#[cfg(test)]
mod test {
    use super::Riven;
    use crate::market::{
        Queryable,
        models::ResponseBase,
    };

    #[rstest::rstest]
    fn riven(
        #[files("src/market/models/fixtures/riven.json")]
        #[mode = str]
        json: &str,
    ) -> Result<(), serde_json::Error> {
        serde_json::from_str::<ResponseBase<<Riven as Queryable>::Data>>(json)?;

        Ok(())
    }
}
