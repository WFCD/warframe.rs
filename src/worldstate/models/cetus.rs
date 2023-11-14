use crate::{enum_builder, model_builder};

enum_builder! {
    CetusState;
    Day = "day",
    Night = "night",
}

model_builder! {
    Cetus: "/cetusCycle",
    rt = obj,
    timed = true;
    pub id: String,
    pub state: CetusState,
}
