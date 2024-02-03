use std::error::Error;

use reqwest::Response;
use serde::{Deserialize, Deserializer};

/// The `OriginalError` struct represents an error with a string message and an error code.
/// This is "as is", meaning this is how the API returns this error.
#[derive(Debug, Deserialize)]
struct OriginalError {
    /// The error message
    error: String,
    // The status code returned
    code: u16,
}

#[derive(Debug)]
pub enum ApiError {
    /// This options represents the status code `400`
    BadRequest(String),

    /// This options represents the status code `500`
    InternalServerError(String),

    /// This options represents the status code `404`
    NotFound(String),

    /// This options represents everything else that isn't specifically implemented
    Other(String, u16),

    /// This option represents the case when the API did not respond with a valid JSON
    ConversionFailed,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Worldstate Api Error")
    }
}

impl Error for ApiError {}

impl<'de> Deserialize<'de> for ApiError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let original_error = OriginalError::deserialize(deserializer)?;

        match original_error.code {
            400 => Ok(ApiError::BadRequest(original_error.error)),
            404 => Ok(ApiError::NotFound(original_error.error)),
            500 => Ok(ApiError::InternalServerError(original_error.error)),
            _ => Ok(ApiError::Other(original_error.error, original_error.code)),
        }
    }
}

impl ApiError {
    pub async fn from(response: Response) -> Self {
        let deserialed_error = response.json::<Self>().await;

        match deserialed_error {
            Ok(res) => res,
            Err(_) => Self::ConversionFailed,
        }
    }
}
