//! Here lies what powers the models.

use std::{
    fmt::Write,
    ops::{
        Div,
        Rem,
    },
};

use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    de::DeserializeOwned,
};

use crate::worldstate::language::Language;

/// Types implementing this have an actual endpoint on the API.
pub trait Endpoint {
    /// Returns the URL to the english endpoint.
    ///
    /// Getting the english endpoint is free, as concatenating is done at compile time.
    fn endpoint_en(base_url: &str) -> String;

    /// Returns the URL to the endpoint of the specified language.
    ///
    /// Getting this endpoint is __NOT__ free, as concatenating is done at runtime.
    fn endpoint(base_url: &str, language: crate::worldstate::language::Language) -> String;
}

/// The `TimedEvent` trait defines methods that are related to timed events
pub trait TimedEvent {
    /// The time when the event began
    fn activation(&self) -> DateTime<Utc>;

    /// The time when the event ends
    fn expiry(&self) -> DateTime<Utc>;

    /// Short-time-formatted duration string representing the start of the event
    fn start_string(&self) -> String {
        format!("-{}", get_short_format_time_string(self.activation()))
    }

    /// Short-time-formatted duration string representing the end of the event
    fn eta(&self) -> String {
        get_short_format_time_string(self.expiry())
    }

    /// Whether the event is expired or not
    fn expired(&self) -> bool {
        chrono::Utc::now() >= self.expiry()
    }

    /// Whether the event is active or not
    fn active(&self) -> bool {
        let now = chrono::Utc::now();
        now >= self.activation() && now <= self.expiry()
    }

    /// Whether the event is still upcoming or not
    fn upcoming(&self) -> bool {
        !self.active() && !self.expired()
    }
}

/// Marks a struct as `Queryable`.
///
/// Comes with a default implementation that works universally.
pub trait Queryable: Endpoint {
    /// The Type returned by the [query](Queryable::query).
    type Return: DeserializeOwned;

    /// Queries a model and returns an instance of [`itself`](Queryable::Return).
    #[must_use]
    fn query(
        base_url: &str,
        request_executor: &reqwest::Client,
    ) -> impl std::future::Future<Output = Result<Self::Return, Error>> + Send {
        async {
            Ok(request_executor
                .get(Self::endpoint_en(base_url))
                .send()
                .await?
                .json::<Self::Return>()
                .await?)
        }
    }

    /// Queries a model with the specified language and returns an instance of
    /// [`itself`](Queryable::Return).
    #[must_use]
    fn query_with_language(
        base_url: &str,
        request_executor: &reqwest::Client,
        language: Language,
    ) -> impl std::future::Future<Output = Result<Self::Return, Error>> + Send {
        async move {
            Ok(request_executor
                .get(Self::endpoint(base_url, language))
                .send()
                .await?
                .json::<Self::Return>()
                .await?)
        }
    }
}

fn divmod<T>(number: T, other: T) -> (T, T)
where
    T: Rem<Output = T> + Div<Output = T> + Copy,
{
    (number / other, number % other)
}

pub(crate) fn deserialize_f32_from_string<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

/// The `get_short_format_time_string` function takes a `DateTime` object and returns a formatted
/// string representing the time difference between the current time and the provided time.
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
            write!(formatted_time, "{div_time}{suffix} ").expect("Error `write!`-ing into string");
            time_in_between = mod_time;
        }
    }

    formatted_time.trim().to_string()
}

/// A trait that allows enums with 2 variants to easily access the other variant.
pub trait Opposite {
    /// Returns the opposite of this state
    #[must_use]
    fn opposite(&self) -> Self;
}

use crate::worldstate::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        use chrono::{
            Duration,
            Utc,
        };
        // Example usage
        let event_time = Utc::now() + Duration::hours(5) + Duration::minutes(30);
        let formatted_time = get_short_format_time_string(event_time);

        let assert_result = formatted_time == "5h 29m 59s"
            || formatted_time == "5h 29m 58s"
            || formatted_time == "5h 30m";
        assert!(assert_result);
    }
}
