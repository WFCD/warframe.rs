use super::macros::model_builder;

model_builder! {
    :"An Item in Baro's inventory"
    VoidTraderInventoryItem,
    rt = obj,
    timed = false;

    :"The item that is being sold"
    pub item: String,

    :"The Ducat cost of this item"
    pub ducats: i32,

    :"The Credit cost of this item"
    pub credits: i32,
}

model_builder! {
    :"Information on the current Void Trader offerings, or when he will arrive"
    VoidTrader: "/voidTrader",
    rt = obj,
    timed = true;

    :"Unique identifier for this object/event/thing"
    pub id: String,

    :"The i18n of the Node Baro will appear on"
    pub location: String,

    :"Baro's Inventory"
    pub inventory: Vec<VoidTraderInventoryItem>
}

#[cfg(test)]
mod test {
    use super::VoidTrader;
    use crate::worldstate::{
        client::Client,
        error::ApiError,
    };

    #[cfg(not(feature = "multilangual"))]
    #[tokio::test]
    async fn test_voidtrader() -> Result<(), ApiError> {
        let client = Client::new();

        match client.fetch::<VoidTrader>().await {
            Ok(_voidtrader) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[cfg(feature = "multilangual")]
    #[tokio::test]
    async fn test_voidtrader_ml() -> Result<(), ApiError> {
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_using_lang::<VoidTrader>(Language::ZH).await {
            Ok(_voidtrader) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
