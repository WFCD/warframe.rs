use super::macros::model_builder;

model_builder! {
    :"Popular Deals, discounts, featured deals"
    FlashSale: "/flashSales",
    rt = array,
    timed = true;

    :"The item being sold"
    pub item: String,

    :"The discount of the Item"
    pub discount: i32,

    :"The PLATINUM price of this item"
    pub premium_override: i32,

    :"The CREDIT price of this item"
    pub regular_override: i32,

    :"Whether the item is popular or not"
    pub is_popular: Option<bool>,

    :"Whether the item is featured or not"
    pub is_featured: Option<bool>
}

#[cfg(test)]
mod test {
    use super::FlashSale;
    use crate::worldstate::{
        client::Client,
        error::Error,
    };

    
    #[tokio::test]
    async fn test_flashsale() -> Result<(), Error> {
        let client = Client::new();

        match client.fetch::<FlashSale>().await {
            Ok(_flashsales) => Ok(()),
            Err(why) => Err(why),
        }
    }

    
    #[tokio::test]
    async fn test_flashsale_ml() -> Result<(), Error> {
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_using_lang::<FlashSale>(Language::ZH).await {
            Ok(_flashsales) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
