use warframe_macros::model;

/// Info about the Daily Deal(s)
#[model(endpoint = "/dailyDeals", return_style = Array, timed)]
pub struct DailyDeal {
    /// The Item being sold
    pub item: String,

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
mod test {
    use super::DailyDeal;
    use crate::worldstate::{
        client::Client,
        error::Error,
    };

    #[tokio::test]
    async fn test_dailydeal() -> Result<(), Error> {
        let client = Client::new();

        match client.fetch::<DailyDeal>().await {
            Ok(_dailydeals) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[tokio::test]
    async fn test_dailydeal_ml() -> Result<(), Error> {
        use crate::worldstate::language::Language;

        let client = Client::new();

        match client.fetch_using_lang::<DailyDeal>(Language::ZH).await {
            Ok(_dailydeals) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
