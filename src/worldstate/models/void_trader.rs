use warframe_macros::model;

use super::ItemStringWrapper;

/// An Item in Baro's inventory
#[model]
pub struct VoidTraderInventoryItem {
    /// The item that is being sold
    pub item: ItemStringWrapper,

    /// The Ducat cost of this item
    pub ducats: i32,

    /// The Credit cost of this item
    pub credits: i32,
}

/// Information on the current Void Trader offerings, or when he will arrive
#[model(endpoint = "/voidTrader", return_style = Object, timed)]
pub struct VoidTrader {
    /// Unique identifier for this object/event/thing
    pub id: String,

    /// The i18n of the Node Baro will appear on
    pub location: String,

    /// Baro's Inventory
    pub inventory: Vec<VoidTraderInventoryItem>,
}

#[cfg(test)]
mod test_void_trader {
    use rstest::rstest;
    use serde_json::from_str;

    use super::VoidTrader;
    use crate::worldstate::{
        fixtures::void_trader::{
            void_trader,
            void_trader_en,
        },
        models::Queryable,
    };

    type R = <VoidTrader as Queryable>::Return;

    #[rstest]
    fn test(void_trader_en: &str) {
        from_str::<R>(void_trader_en).unwrap();
    }

    #[rstest]
    fn test_ml(void_trader: &str) {
        from_str::<R>(void_trader).unwrap();
    }
}
