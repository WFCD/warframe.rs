use derive_more::Display;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Display)]
#[serde(rename_all = "lowercase")]
pub enum RivenGroup {
    Primary,
    Secondary,
    Melee,
    Zaw,
    Archgun,
    Kitgun,
    Sentinel,
}
