//! Definitions for errors
use serde::Deserialize;

/// The `OriginalError` struct represents an error with a string message and an error code.
/// This is "as is", meaning this is how the API returns this error.
#[derive(Debug, Deserialize, thiserror::Error)]
#[error("{error} [CODE {code}]")]
pub struct ApiErrorResponse {
    /// The error message
    pub error: String,
    /// The status code returned
    pub code: u16,
}

/// The Error type of this crate
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// An error from the sent request
    #[error("Couldn't send request: {0}")]
    FaultyRequest(#[from] reqwest::Error),

    /// An error that occurs when the deserialization of serde_json fails
    #[error("Couldn't deserialize json body: {0}")]
    FailedDeserialization(#[from] serde_json::Error),

    /// Any error directly from the API
    #[error("Error response from the API: {0}")]
    ApiError(#[from] ApiErrorResponse),
}
