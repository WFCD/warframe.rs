use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Activity {
    /// Name of the activity (e.g., 'on mission', 'dojo').
    pub r#type: ActivityType,
    /// Optional specifics about the activity (e.g., mission name, solo/squad status).
    pub details: Option<String>,
    /// Timestamp of the activity start.
    pub started_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ActivityType {
    /// Represents a mission activity.
    OnMission,
    /// Represents a dojo activity.
    Dojo,

    #[serde(rename = "UNKNOWN")]
    Unknown,

    #[serde(rename = "")]
    Empty,
}
