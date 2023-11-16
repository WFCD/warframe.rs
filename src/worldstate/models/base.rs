use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde_json::error::Error;

pub trait Endpoint {
    fn endpoint() -> &'static str;
}

/// The base trait implemented by every Model in the API.
pub trait Model: DeserializeOwned {
    fn from_str(raw_json: &str) -> Result<Self, Error> {
        serde_json::from_str::<Self>(raw_json)
    }
}

/// The `TimedEvent` trait defines methods that are related to timed events
pub trait TimedEvent {
    /// The time when the event began
    fn activation(&self) -> DateTime<Utc>;

    /// The time when the event ends
    fn expiry(&self) -> DateTime<Utc>;

    /// Short-time-formatted duration string representing the start of the event
    fn start_string(&self) -> String;

    /// Short-time-formatted duration string representing the end of the event
    fn eta(&self) -> String;

    /// Whether the event is expired or not
    fn expired(&self) -> bool;
}

/// Defines a type to be of *API* return type object
pub trait RTObject {}

/// Defines a type to be of *API* return type array
pub trait RTArray {}

/// The `get_short_format_time_string` function takes a `DateTime` object and returns a formatted string
/// representing the time difference between the current time and the provided time.
///
/// Arguments:
///
/// * `dt`: A `chrono::DateTime` object representing a specific date and time in UTC.
///
/// Returns:
///
/// a formatted string representing the time difference between the current time and the provided
/// `DateTime` value.
pub(crate) fn get_short_format_time_string(dt: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let time_in_between = if now > dt { now - dt } else { dt - now };

    let days = time_in_between.num_days();
    let hours = time_in_between.num_hours();
    let minutes = time_in_between.num_minutes();
    let seconds = time_in_between.num_seconds();

    let hours_remainder = hours.rem_euclid(24);
    let minutes_remainder = minutes.rem_euclid(60);
    let seconds_remainder = seconds.rem_euclid(60);

    let time_components = vec![
        (days, "d"),
        (hours_remainder, "h"),
        (minutes_remainder, "m"),
        (seconds_remainder, "s"),
    ];
    let mut formatted_time = String::new();

    for (t, suffix) in time_components {
        if t != 0 {
            formatted_time.push_str(&format!("{}{}", t, suffix));
            formatted_time.push(' ');
        }
    }

    formatted_time.trim().to_string()
}

pub trait Documentation {
    /// Gets the documentation for this variant
    fn docs(&self) -> &'static str;
}

pub trait TypeDocumentation {
    /// Gets the documentation for this Enum
    fn docs() -> &'static str;
}

// #[test]
// fn test_format() {
//     use chrono::{Duration, Utc};
//     // Example usage
//     let event_time = Utc::now() + Duration::hours(5) + Duration::minutes(30);
//     let formatted_time = get_short_format_time_string(event_time);

//     let assert_result = formatted_time == "5h 29m 59s" || formatted_time == "5h 30m 60s";
//     assert!(assert_result)
// }
