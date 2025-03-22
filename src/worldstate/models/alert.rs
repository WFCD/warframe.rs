use warframe_macros::model;

use super::{
    mission::Mission,
    reward_type::RewardType,
};

/// An alert in Warframe
#[model(endpoint = "/alerts", return_style = Array)]
pub struct Alert {
    /// ID of this event
    pub id: String,

    /// The mission associated with the alert
    pub mission: Mission,

    /// The reward type of the alert
    pub reward_types: Vec<RewardType>,
}

#[cfg(test)]
mod test_alert {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Alert;
    use crate::worldstate::{
        Queryable,
        fixtures::alert::{
            alert,
            alert_en,
        },
    };

    type R = <Alert as Queryable>::Return;

    #[rstest]
    fn test(alert_en: &str) {
        from_str::<R>(alert_en).unwrap();
    }

    #[rstest]
    fn test_ml(alert: &str) {
        from_str::<R>(alert).unwrap();
    }
}
