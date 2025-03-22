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
    use crate::worldstate::{
        Queryable,
        fixtures::global_upgrade::{
            global_upgrade,
            global_upgrade_en,
        },
    };

    type R = <GlobalUpgrade as Queryable>::Return;

    #[rstest]
    fn test(global_upgrade_en: &str) {
        from_str::<R>(global_upgrade_en).unwrap();
    }

    #[rstest]
    fn test_ml(global_upgrade: &str) {
        from_str::<R>(global_upgrade).unwrap();
    }
}
