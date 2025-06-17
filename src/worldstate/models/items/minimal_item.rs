use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MinimalItem {
    pub description: String,

    pub image_name: String,

    pub masterable: bool,

    pub name: String,

    pub tradable: bool,

    pub unique_name: String,
}
