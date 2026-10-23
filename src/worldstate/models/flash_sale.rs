use warframe_macros::model;

use super::ItemStringWrapper;

/// Popular Deals, discounts, featured deals
#[model(endpoint = "/flashSales", return_style = Array, timed)]
pub struct FlashSale {
    /// The item being sold
    pub item: ItemStringWrapper,

    /// The unique name of the item
    pub unique_name: String,

    /// The discount of the Item
    pub discount: Option<i32>,

    /// The PLATINUM price of this item
    pub premium_override: Option<i32>,

    /// The CREDIT price of this item
    pub regular_override: Option<i32>,

    /// Whether the item is shown in market
    pub is_shown_in_market: Option<bool>,
}

#[cfg(test)]
mod test_flash_sale {
    use rstest::rstest;
    use serde_json::from_str;

    use super::FlashSale;
    use crate::worldstate::Queryable;

    type R = <FlashSale as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/flash_sale.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
