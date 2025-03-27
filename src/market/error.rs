use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Reqwest(#[from] reqwest::Error),
    Serde(#[from] serde_json::Error),

    #[error("API responded with error: {0}")]
    Api(String),

    /// The API has an error field, which may be empty.
    /// If the error field is empty, the data field should not be empty.
    ///
    /// This error represents the case where the both fields are empty. (should not happen though)
    #[error("API responded with both an empty error and empty data")]
    EmptyErrorAndData,
}

pub type Result<T> = std::result::Result<T, Error>;
