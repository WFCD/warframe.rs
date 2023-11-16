use super::macros::{enum_builder, model_builder};

enum_builder! {
    :"Represents the current state on cetus"
    CetusState;

    :"Represents Cetus' day state"
    Day = "day",

    :"Rpresents Cetus' night state"
    Night = "night",
}

impl CetusState {
    /// Returns the other state of the state inputted.
    ///
    /// For example:
    ///
    /// Day -> Night / Night -> Day
    pub fn opposite(&self) -> Self {
        match self {
            CetusState::Day => CetusState::Night,
            CetusState::Night => CetusState::Day,
        }
    }
}

model_builder! {
    :"The Information about cetus"
    Cetus: "/cetusCycle",
    rt = obj,
    timed = true;

    :"The id of the rotation"
    pub id: String,

    :"The state of Cetus (day/night)"
    pub state: CetusState,
}
