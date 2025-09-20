use warframe_macros::model;

/// Represents the state on Orb Vallis
#[model]
#[serde(rename_all = "lowercase")]
pub enum OrbVallisState {
    /// Warm
    Warm,
    /// Cold
    Cold,
}

/// The current cycle of the Orb Vallis warm/cold cycle
#[model(endpoint = "/vallisCycle", return_style = Object, timed)]
pub struct OrbVallis {
    /// Unique identifier for this object/event/thing
    pub id: String,

    /// The state of the orb vallis (warm/cold)
    pub state: OrbVallisState,
}

#[cfg(test)]
mod test_orb_vallis {
    use rstest::rstest;
    use serde_json::from_str;

    use super::OrbVallis;
    use crate::worldstate::Queryable;

    type R = <OrbVallis as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/orb_vallis.json")]
        #[mode = str]
        orb_vallis_en: &str,
    ) {
        from_str::<R>(orb_vallis_en).unwrap();
    }
}
