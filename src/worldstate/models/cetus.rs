use super::macros::{enum_builder, model_builder};
enum_builder! {
    CetusState;
    Day = "day",
    Night = "night",
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
