//! This module defines error types

/// The market's error type
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// An error from the sent request
    #[error("Couldn't send request: {0}")]
    FaultyRequest(#[from] reqwest::Error),

    /// An error that occurs when the deserialization of `serde_json` fails
    #[error("Couldn't deserialize json body: {0}")]
    FailedDeserialization(#[from] serde_json::Error),

    /// Any error directly from the API (status code only)
    #[error("Error response from the API: {0}")]
    ApiError(reqwest::StatusCode),
}

impl From<reqwest::StatusCode> for ApiError {
    fn from(value: reqwest::StatusCode) -> Self {
        ApiError::ApiError(value)
    }
}
