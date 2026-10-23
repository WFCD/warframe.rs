use chrono::{
    DateTime,
    Utc,
};
use warframe_macros::model;

/// A challenge on the 1999 calendar
#[model]
pub struct CalendarChallenge {
    /// The title of the challenge
    pub title: String,

    /// The description of the challenge
    pub description: String,
}

/// An upgrade/override on the 1999 calendar
#[model]
pub struct CalendarUpgrade {
    /// The title of the upgrade
    pub title: String,

    /// The description of the upgrade
    pub description: String,
}

/// An event on the 1999 calendar
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(tag = "type")]
pub enum CalendarEvent {
    /// A challenge to complete
    #[serde(rename = "To Do")]
    ToDo {
        /// The challenge details
        challenge: CalendarChallenge,
    },

    /// A reward to claim
    #[serde(rename = "Big Prize!")]
    BigPrize {
        /// The reward description
        reward: String,
        /// The unique name of the reward item
        #[serde(rename = "uniqueName")]
        unique_name: String,
    },

    /// A buff/upgrade override
    #[serde(rename = "Override")]
    Override {
        /// The upgrade details
        upgrade: CalendarUpgrade,
    },
}

/// A day on the 1999 calendar
#[model]
pub struct CalendarDay {
    /// The date
    pub date: DateTime<Utc>,

    /// Events happening on this day
    pub events: Vec<CalendarEvent>,
}

/// The 1999 Calendar showing daily events, challenges, and rewards
#[model(endpoint = "/calendar", return_style = Object, timed)]
pub struct Calendar {
    /// The current season (e.g. "Winter")
    pub season: Option<String>,

    /// The year iteration
    pub year_iteration: Option<i32>,

    /// The version of the calendar
    pub version: Option<serde_json::Value>,

    /// Requirements to access the calendar
    #[serde(default)]
    pub requirements: Vec<String>,

    /// The days of the calendar
    pub days: Vec<CalendarDay>,
}

#[cfg(test)]
mod test_calendar {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Calendar;
    use crate::worldstate::Queryable;

    type R = <Calendar as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/calendar.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
