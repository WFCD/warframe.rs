use chrono::{
    DateTime,
    Utc,
};
use serde::Deserialize;

use super::activity::Activity;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct UserShort {
    pub id: String,
    /// In-game name of the user.
    pub ingame_name: String,
    /// Optional avatar image.
    pub avatar: Option<String>,
    /// Reputation score.
    pub reputation: u16,
    /// Preferred communication language (e.g., 'en', 'ko', 'es').
    pub locale: String,
    /// Gaming platform used by the user.
    pub platform: String,
    pub crossplay: bool,

    /// Current status of the user.
    pub status: Status,
    /// Addition to the status, current activity of the user.
    pub activity: Activity,
    /// Timestamp of the user's last online presence.
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
#[serde(rename_all = "snake_case")]
/// Represents the status of a user.
pub enum Status {
    Ingame,
    Online,
    Offline,
}
