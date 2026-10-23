use warframe_macros::model;

use super::void_trader::VoidTraderInventoryItem;

/// Information on the current Vault Trader (Varzia) offerings
#[model(endpoint = "/vaultTrader", return_style = Object, timed)]
pub struct VaultTrader {
    /// Unique identifier for this object/event/thing
    pub id: String,

    /// The name of the trader
    pub character: Option<String>,

    /// The i18n of the Node the trader will appear on
    pub location: String,

    /// The trader's inventory
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
mod test_vault_trader {
    use rstest::rstest;
    use serde_json::from_str;

    use super::VaultTrader;
    use crate::worldstate::Queryable;

    type R = <VaultTrader as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/vault_trader.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
