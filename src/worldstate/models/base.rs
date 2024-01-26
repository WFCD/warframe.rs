use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde_json::error::Error;

use std::ops::{Div, Rem};

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

fn divmod<T>(number: T, other: T) -> (T, T)
where
    T: Rem<Output = T> + Div<Output = T> + Copy,
{
    (number / other, number % other)
}

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
pub(crate) fn get_short_format_time_string(dt: DateTime<Utc>) -> String {
    let now = Utc::now();
    let mut time_in_between = (if now > dt { now - dt } else { dt - now }).num_seconds();

    let components = [("h", 60 * 60), ("m", 60), ("s", 1)];

    let mut formatted_time = String::new();

    for &(suffix, divisor) in &components {
        let (div_time, mod_time) = divmod(time_in_between, divisor);
        if div_time > 0 {
            formatted_time.push_str(&format!("{}{} ", div_time, suffix));
            time_in_between = mod_time;
        }
    }

    formatted_time.trim().to_string()
}

pub trait VariantDocumentation {
    /// Gets the documentation for this variant
    fn docs(&self) -> &'static str;
}

pub trait TypeDocumentation {
    /// Gets the documentation for this Enum
    fn docs() -> &'static str;
}

pub trait Opposite {
    #[doc = "Returns the opposite of this state"]
    fn opposite(&self) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        use chrono::{Duration, Utc};
        // Example usage
        let event_time = Utc::now() + Duration::hours(5) + Duration::minutes(30);
        let formatted_time = get_short_format_time_string(event_time);

        let assert_result = formatted_time == "5h 29m 59s"
            || formatted_time == "5h 29m 58s"
            || formatted_time == "5h 30m";
        assert!(assert_result)
    }
}
