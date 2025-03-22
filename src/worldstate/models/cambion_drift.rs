use warframe_macros::model;

/// The State of the Cambion Drift
#[model]
#[serde(rename_all = "lowercase")]
pub enum CambionDriftState {
    /// The 'Vome' state
    Vome,
    /// The 'Fass' state
    Fass,
}

/// Cambion Drift info
#[model(endpoint = "/cambionCycle", return_style = Object, timed)]
pub struct CambionDrift {
    /// The id of the cycle
    pub id: String,

    /// The state of the cambion drift (vome/fass)
    pub state: CambionDriftState,
}

#[cfg(test)]
mod test_cambion_drift {
    use rstest::rstest;
    use serde_json::from_str;

    use super::CambionDrift;
    use crate::worldstate::{
        Queryable,
        fixtures::cambion_drift::{
            cambion_drift,
            cambion_drift_en,
        },
    };

    type R = <CambionDrift as Queryable>::Return;

    #[rstest]
    fn test(cambion_drift_en: &str) {
        from_str::<R>(cambion_drift_en).unwrap();
    }

    #[rstest]
    fn test_ml(cambion_drift: &str) {
        from_str::<R>(cambion_drift).unwrap();
    }
}
