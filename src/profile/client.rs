//! Provides a client that acts as the baseline for interacting with the profile API

#[derive(Default, Debug, Clone)]
pub struct Client {
    session: reqwest::Client,
}

impl Client {
    /// Creates a new [Client].
    pub fn new() -> Self {
        Default::default()
    }
}
