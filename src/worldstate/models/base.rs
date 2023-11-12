use serde::de::DeserializeOwned;
use serde_json::error::Error;

pub trait Endpoint {
    fn endpoint() -> &'static str;
}

pub trait Model: DeserializeOwned {
    fn from_str(raw_json: &str) -> Result<Self, Error> {
        serde_json::from_str::<Self>(raw_json)
    }
}

pub trait TimedEvent {
    /// Short-time-formatted duration string representing the start of the event
    fn start_string(&self) -> String;

    /// Short-time-formatted duration string representing the end of the event
    fn eta(&self) -> String;

    /// Whether the event is expired or not
    fn expired(&self) -> bool;
}

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

#[test]
fn test_format() {
    use chrono::{Duration, Utc};
    // Example usage
    let event_time = Utc::now() + Duration::hours(5) + Duration::minutes(30);
    let formatted_time = get_short_format_time_string(event_time);

    let assert_result = formatted_time == "5h 29m 59s" || formatted_time == "5h 30m 60s";
    assert!(assert_result)
}
