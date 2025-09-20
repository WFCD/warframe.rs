use warframe_macros::model;

/// Any current modifiers applied to all users, such as double drops, double XP, etc.
#[model(endpoint = "/globalUpgrades", return_style = Array, timed)]
pub struct GlobalUpgrade {
    /// What kind of upgrade
    pub upgrade: String,

    /// Operation descriptor
    pub operation: String,

    /// Symbol corresponding to operation
    pub operation_symbol: String,

    /// Value corresponding to performing the operation
    pub upgrade_operation_value: i32,

    /// Whether the upgrade has expired
    pub expired: bool,
}

#[cfg(test)]
mod test_global_upgrade {
    use rstest::rstest;
    use serde_json::from_str;

    use super::GlobalUpgrade;
    use crate::worldstate::Queryable;

    type R = <GlobalUpgrade as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/global_upgrade.json")]
        #[mode = str]
        global_upgrade_en: &str,
    ) {
        from_str::<R>(global_upgrade_en).unwrap();
    }
}
