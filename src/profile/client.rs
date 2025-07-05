//! Provides a client that acts as the baseline for interacting with the profile API

use super::{
    error::Error,
    models::platform::Platform,
};
use crate::profile::models::profile::Profile;

#[derive(Default, Debug, Clone)]
pub struct Client {
    session: reqwest::Client,
}

impl Client {
    /// Creates a new [Client].
    #[must_use]
    pub fn new() -> Self {
        Client::default()
    }
}

impl Client {
    /// Fetches the profile of the user with the given username.
    #[allow(clippy::missing_errors_doc)]
    pub async fn fetch(&self, player_id: &str, platform: Platform) -> Result<Profile, Error> {
        let url = format!(
            "https://{}.warframe.com/dynamic/getProfileViewingData.php?playerId={player_id}",
            platform.to_sub_domain(),
        );
        let response = self.session.get(&url).send().await?;

        if response.status().is_success() {
            let profile = response.json::<Profile>().await?;
            Ok(profile)
        } else if response.status() == reqwest::StatusCode::CONFLICT {
            let response = response.text().await?;
            let split = response.split(' ').collect::<Vec<&str>>();
            Err(Error::WrongPlatform(split[2].to_string()))
        } else {
            Err(Error::ApiError(response.status()))
        }
    }
}
