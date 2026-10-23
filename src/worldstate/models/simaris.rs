use warframe_macros::model;

/// Cephalon Simaris' current synthesis target
#[model(endpoint = "/simaris", return_style = Object)]
pub struct Simaris {
    /// The current synthesis target
    pub target: String,

    /// Whether the target is currently active
    pub is_target_active: bool,
}

#[cfg(test)]
mod test_simaris {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Simaris;
    use crate::worldstate::Queryable;

    type R = <Simaris as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/simaris.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
