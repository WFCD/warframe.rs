use chrono::{
    DateTime,
    Utc,
};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    Sell,
    Buy,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// Is the unique identifier of the order.
    pub id: String,

    /// Specifies whether the order is a 'buy' or 'sell'.
    pub r#type: OrderType,

    /// Is the total platinum currency involved in the order.
    pub platinum: u32,

    /// Represents the number of items included in the order.
    pub quantity: u32,

    /// (optional) indicates the items quantity per transaction.
    pub per_trade: Option<u8>,

    /// (optional) specifies the rank or level of the item in the order.
    pub rank: Option<u8>,

    /// (optional) specifies number of charges left (used in requiem mods).
    pub charges: Option<u8>,

    /// (optional) defines the specific subtype or category of the item.
    pub subtype: Option<String>,

    /// (optional) denotes the count of amber stars in a sculpture order.
    pub amber_stars: Option<u8>,

    /// (optional) denotes the count of cyan stars in a sculpture order.
    pub cyan_stars: Option<u8>,

    /// (auth\mod) Indicates whether the order is publicly visible or not.
    pub visible: bool,

    /// Records the creation time of the order.
    pub created_at: DateTime<Utc>,

    /// Records the last modification time of the order.
    pub updated_at: DateTime<Utc>,

    /// Is the unique identifier of the item involved in the order.
    pub item_id: String,

    /// User-defined group to which the order belongs
    pub group: Option<String>,
}
