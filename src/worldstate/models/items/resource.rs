use serde::Deserialize;

use super::Drop;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    description: String,

    drops: Vec<Drop>,

    image_name: String,

    item_count: i64,

    masterable: bool,

    name: String,

    parents: Vec<String>,

    tradable: bool,

    r#type: String,

    unique_name: String,
}
