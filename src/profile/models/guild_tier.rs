use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum GuildTier {
    Ghost = 1,
    Shadow = 2,
    Storm = 3,
    Mountain = 4,
    Moon = 5,
}
