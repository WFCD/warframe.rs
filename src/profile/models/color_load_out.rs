use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
/// base10 i32 color loadout, [None] if color is not set
pub struct ColorLoadOut {
    #[serde(rename = "t0")]
    /// primary
    pub t0: Option<i32>,

    #[serde(rename = "t1")]
    /// secondary
    pub t1: Option<i32>,

    #[serde(rename = "t2")]
    /// tertiary
    pub t2: Option<i32>,

    #[serde(rename = "t3")]
    /// accents
    pub t3: Option<i32>,

    #[serde(rename = "m0")]
    /// emissive_primary
    pub m0: Option<i32>,

    #[serde(rename = "m1")]
    /// emissive_secondary
    pub m1: Option<i32>,

    #[serde(rename = "en")]
    /// energy_primary
    pub en: Option<i32>,

    #[serde(rename = "e1")]
    /// energy_secondary
    pub e1: Option<i32>,
}
