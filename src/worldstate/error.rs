use serde::Deserialize;

/// The `OriginalError` struct represents an error with a string message and an error code.
/// This is "as is", meaning this is how the API returns this error.
#[derive(Debug, Deserialize, thiserror::Error)]
#[error("")]
pub struct ApiErrorResponse {
    /// The error message
    pub error: String,
    // The status code returned
    pub code: u16,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Couldn't send request: {0}")]
    FaultyRequest(#[from] reqwest::Error),

    #[error("Couldn't deserialize json body: {0}")]
    FailedDeserialization(#[from] serde_json::Error),

    #[error("Error response from the API: {0}")]
    ApiError(#[from] ApiErrorResponse),
}
