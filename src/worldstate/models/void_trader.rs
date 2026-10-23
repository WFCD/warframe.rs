use warframe_macros::model;

use super::ItemStringWrapper;

/// An Item in a Void Trader's inventory
#[model]
pub struct VoidTraderInventoryItem {
    /// The unique name identifier of the item
    pub unique_name: Option<String>,

    /// The item that is being sold
    pub item: ItemStringWrapper,

    /// The Ducat cost of this item
    pub ducats: Option<i32>,

    /// The Credit cost of this item
    pub credits: Option<i32>,
}

/// Information on the current Void Trader offerings, or when he will arrive
#[model(endpoint = "/voidTrader", return_style = Object, timed)]
pub struct VoidTrader {
    /// Unique identifier for this object/event/thing
    pub id: String,

    /// The name of the trader
    pub character: Option<String>,

    /// The i18n of the Node the trader will appear on
    pub location: String,

    /// The trader's Inventory
    pub inventory: Vec<VoidTraderInventoryItem>,

    /// Platform-specific identifier
    pub ps_id: Option<String>,

    /// The initial start date
    pub initial_start: Option<String>,

    /// The schedule of the trader
    #[serde(default)]
    pub schedule: Vec<serde_json::Value>,
}

#[cfg(test)]
mod test_void_trader {
    use rstest::rstest;
    use serde_json::from_str;

    use super::VoidTrader;
    use crate::worldstate::Queryable;

    type R = <VoidTrader as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/void_trader.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
