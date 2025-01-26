//! Provides a client that acts as the baseline for interacting with the profile API

use super::{
    error::ApiError,
    models::{
        platform::Platform,
        profile::Profile
    }
};

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

impl Client {
    /// Fetches the profile of the user with the given username.
    pub async fn fetch(&self, username: &str, platform: Platform) -> Result<Profile, ApiError> {
        let url = format!("https://{}.warframe.com/dynamic/getProfileViewingData.php?n={}", platform.to_sub_domain(), username);
        let response = self.session.get(&url).send().await?;

        if response.status().is_success() {
            let profile = response.json::<Profile>().await?;
            Ok(profile)
        } else if response.status() == reqwest::StatusCode::CONFLICT {
            let split = response.text().await?.split(" ").collect::<Vec<&str>>();
            Err(ApiError::WrongPlatform(split[2].to_string()))
        } else {
            Err(ApiError::ApiError(response.status()))
        }
    }
}
