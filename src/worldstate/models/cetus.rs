use super::{base::Endpoint, macros::timed_event};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename(deserialize = "state"))]
pub enum CetusState {
    #[serde(rename(deserialize = "day"))]
    Day,

    #[serde(rename(deserialize = "night"))]
    Night,
}

timed_event!(Cetus
    pub state: CetusState
);

impl Endpoint for Cetus {
    fn get_endpoint() -> &'static str {
        "/cetusCycle"
    }
}
