//! Here lies what powers the models.

use std::ops::{
    Div,
    Rem,
};

use chrono::{
    DateTime,
    Utc,
};
use serde::{
    de::DeserializeOwned,
    Deserialize,
};

/// Types implementing this have an actual endpoint on the API.
pub trait Endpoint {
    /// Returns the URL to the english endpoint.
    ///
    /// Getting the english endpoint is free, as concatenating is done at compile time.
    fn endpoint_en() -> &'static str;

    /// Returns the URL to the endpoint of the specified language.
    ///
    /// Getting this endpoint is __NOT__ free, as concatenating is done at runtime.
    #[cfg(feature = "multilangual")]
    fn endpoint(language: crate::worldstate::language::Language) -> String;
}

/// The `TimedEvent` trait defines methods that are related to timed events
pub trait TimedEvent {
    /// The time when the event began
    fn activation(&self) -> DateTime<Utc>;

    /// The time when the event ends
    fn expiry(&self) -> DateTime<Utc>;

    /// Short-time-formatted duration string representing the start of the event
    fn start_string(&self) -> String {
        format!(
            "-{}",
            super::base::get_short_format_time_string(self.activation())
        )
    }

    /// Short-time-formatted duration string representing the end of the event
    fn eta(&self) -> String {
        super::base::get_short_format_time_string(self.expiry())
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

    /// Queries a model and returns an instance of ['itself'](Queryable::Return).
    fn query(
        request_executor: &reqwest::Client,
    ) -> impl std::future::Future<Output = Result<Self::Return, ApiError>> + Send {
        async {
            Ok(request_executor
                .get(Self::endpoint_en())
                .send()
                .await?
                .json::<Self::Return>()
                .await?)
        }
    }

    /// Queries a model with the specified language and returns an instance of
    /// ['itself'](Queryable::Return).
    #[cfg(feature = "multilangual")]
    fn query_with_language(
        request_executor: &reqwest::Client,
        language: crate::worldstate::prelude::Language,
    ) -> impl std::future::Future<Output = Result<Self::Return, ApiError>> + Send {
        async {
            Ok(request_executor
                .get(Self::endpoint(language))
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
            formatted_time.push_str(&format!("{}{} ", div_time, suffix));
            time_in_between = mod_time;
        }
    }

    formatted_time.trim().to_string()
}

/// A trait allowing to get the documentation of an enum variant - so you don't have to write it.
pub trait VariantDocumentation {
    /// Gets the documentation for this variant
    fn docs(&self) -> &'static str;
}

/// A trait allowing to get the documentation of a type - so you don't have to write it.
pub trait TypeDocumentation {
    /// Gets the documentation for this Enum
    fn docs() -> &'static str;
}

/// A trait that allows enums with 2 variants to easily access the other variant.
pub trait Opposite {
    /// Returns the opposite of this state
    fn opposite(&self) -> Self;
}

use crate::worldstate::error::ApiError;

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
        assert!(assert_result)
    }
}
