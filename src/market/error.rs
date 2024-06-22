#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Couldn't send request: {0}")]
    FaultyRequest(#[from] reqwest::Error),

    #[error("Couldn't deserialize json body: {0}")]
    FailedDeserialization(#[from] serde_json::Error),

    #[error("Error response from the API: {0}")]
    ApiError(reqwest::StatusCode),
}

impl From<reqwest::StatusCode> for ApiError {
    fn from(value: reqwest::StatusCode) -> Self {
        ApiError::ApiError(value)
    }
}
