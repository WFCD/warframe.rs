use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Misc {
    pub description: String,

    pub image_name: String,

    pub masterable: bool,

    pub name: String,

    pub tradable: bool,

    pub r#type: String,

    pub unique_name: String,
}
