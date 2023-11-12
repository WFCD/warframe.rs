use super::macros::model_builder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename(deserialize = "state"))]
pub enum CetusState {
    #[serde(rename(deserialize = "day"))]
    Day,

    #[serde(rename(deserialize = "night"))]
    Night,
}

model_builder! {
    Cetus: "/cetusCycle",
    timed = true;
    pub state: CetusState
}
