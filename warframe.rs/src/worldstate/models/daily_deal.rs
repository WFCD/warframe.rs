use super::macros::model_builder;

model_builder! {
    :"Info about the Daily Deal(s)"
    DailyDeal: "/dailyDeals",
    rt = array,
    timed = true;


    :"The Item being sold"
    pub item: String,

    :"The unique name of the Item"
    pub unique_name: String,

    :"The original price of the Item"
    pub original_price: i32,

    :"The discounted price"
    pub sale_price: i32,

    :"The total amount of items available"
    pub total: i32,

    :"The number of items sold"
    pub sold: i32,

    :"The discount % of the item"
    pub discount: i32,
}

#[cfg(test)]
mod test {
    use super::DailyDeal;
    use crate::worldstate::{client::Client, error::ApiError};

    #[cfg(not(feature = "multilangual"))]
    #[tokio::test]
    async fn test_dailydeal() -> Result<(), ApiError> {
        let client = Client::new();

        match client.fetch_arr::<DailyDeal>().await {
            Ok(_dailydeals) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[cfg(feature = "multilangual")]
    #[tokio::test]
    async fn test_dailydeal_ml() -> Result<(), ApiError> {
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_arr_using_lang::<DailyDeal>(Language::ZH).await {
            Ok(_dailydeals) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
