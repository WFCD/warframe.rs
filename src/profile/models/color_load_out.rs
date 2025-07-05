use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
/// base10 i32 color loadout, [None] if color is not set
pub struct ColorLoadOut {
    #[serde(rename = "t0")]
    pub t0: Option<i32>,

    #[serde(rename = "t1")]
    pub t1: Option<i32>,

    #[serde(rename = "t2")]
    pub t2: Option<i32>,

    #[serde(rename = "t3")]
    pub t3: Option<i32>,

    #[serde(rename = "m0")]
    pub m0: Option<i32>,

    #[serde(rename = "m1")]
    pub m1: Option<i32>,

    #[serde(rename = "en")]
    pub en: Option<i32>,

    #[serde(rename = "e1")]
    pub e1: Option<i32>,
}
