use serde::Deserialize;

use super::Category;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Glyph {
    pub category: Category,

    pub description: String,

    pub image_name: String,

    pub masterable: bool,

    pub name: String,

    pub tradable: bool,

    pub unique_name: String,
}
