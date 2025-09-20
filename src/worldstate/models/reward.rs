use warframe_macros::model;

/// Small info about how many of which items are given
#[model]
pub struct CountedItem {
    /// How many of this item
    pub count: i32,

    /// The item type
    pub r#type: String,
}

/// The reward of this event
#[model]
pub struct Reward {
    /// Items that have a quantity attached
    pub counted_items: Vec<CountedItem>,

    /// Thumbnail URL
    pub thumbnail: String,

    /// RGB value as an int assigned to this reward
    pub color: i32,

    /// Amount of credits awarded
    pub credits: i32,

    /// Items' names possible to be won
    pub items: Vec<String>,
}
