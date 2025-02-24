use warframe_macros::model;

use super::ItemStringWrapper;

/// Popular Deals, discounts, featured deals
#[model(endpoint = "/flashSales", return_style = Array, timed)]
pub struct FlashSale {
    /// The item being sold
    pub item: ItemStringWrapper,

    /// The discount of the Item
    pub discount: i32,

    /// The PLATINUM price of this item
    pub premium_override: i32,

    /// The CREDIT price of this item
    pub regular_override: i32,

    /// Whether the item is popular or not
    pub is_popular: Option<bool>,

    /// Whether the item is featured or not
    pub is_featured: Option<bool>,
}

#[cfg(test)]
mod test_flash_sale {
    use rstest::rstest;
    use serde_json::from_str;

    use super::FlashSale;
    use crate::worldstate::{
        fixtures::flash_sale::{
            flash_sale,
            flash_sale_en,
        },
        models::Queryable,
    };

    type R = <FlashSale as Queryable>::Return;

    #[rstest]
    fn test(flash_sale_en: &str) {
        from_str::<R>(flash_sale_en).unwrap();
    }

    #[rstest]
    fn test_ml(flash_sale: &str) {
        from_str::<R>(flash_sale).unwrap();
    }
}
