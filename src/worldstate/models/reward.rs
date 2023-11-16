use super::macros::model_builder;

model_builder! {
    CountedItem,
    rt = array;

    pub count: i32,
    pub r#type: String
}

model_builder! {
    :"The reward of this event"
    Reward,
    rt = obj;

    :"Items that have a quantity attached"
    pub counted_items: Vec<CountedItem>,

    :"Thumbnail URL"
    pub thumbnail: String,

    :"RGB value as an int assigned to this reward"
    pub color: i32,

    :"Amount of credits awarded"
    pub credits: i32,

    :"string representation of the reward"
    pub as_string: String,

    :"Items' names possible to be won"
    pub items: Vec<String>,

    :"formatted string describing all included items"
    pub item_string: String,
}
