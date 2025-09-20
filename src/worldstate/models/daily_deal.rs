use warframe_macros::model;

use super::ItemStringWrapper;

/// Info about the Daily Deal(s)
#[model(endpoint = "/dailyDeals", return_style = Array, timed)]
pub struct DailyDeal {
    /// The Item being sold
    pub item: ItemStringWrapper,

    /// The unique name of the Item
    pub unique_name: String,

    /// The original price of the Item
    pub original_price: i32,

    /// The discounted price
    pub sale_price: i32,

    /// The total amount of items available
    pub total: i32,

    /// The number of items sold
    pub sold: i32,

    /// The discount % of the item
    pub discount: i32,
}

#[cfg(test)]
mod test_daily_deal {
    use rstest::rstest;
    use serde_json::from_str;

    use super::DailyDeal;
    use crate::worldstate::Queryable;

    type R = <DailyDeal as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/daily_deal.json")]
        #[mode = str]
        daily_deal_en: &str,
    ) {
        from_str::<R>(daily_deal_en).unwrap();
    }
}
