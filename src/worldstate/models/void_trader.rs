use warframe_macros::model;

/// An Item in Baro's inventory
#[model]
pub struct VoidTraderInventoryItem {
    /// The item that is being sold
    pub item: String,

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
mod test {
    use super::VoidTrader;
    use crate::worldstate::{
        client::Client,
        error::Error,
    };

    #[tokio::test]
    async fn test_voidtrader() -> Result<(), Error> {
        let client = Client::new();

        match client.fetch::<VoidTrader>().await {
            Ok(_voidtrader) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[tokio::test]
    async fn test_voidtrader_ml() -> Result<(), Error> {
        use crate::worldstate::language::Language;

        let client = Client::new();

        match client.fetch_using_lang::<VoidTrader>(Language::ZH).await {
            Ok(_voidtrader) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
