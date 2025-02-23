use serde::Deserialize;

use super::Category;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Sigil {
    category: Category,

    description: String,

    image_name: String,

    masterable: bool,

    name: String,

    tradable: bool,

    #[serde(rename = "type")]
    sigil_type: String,

    unique_name: String,
}

