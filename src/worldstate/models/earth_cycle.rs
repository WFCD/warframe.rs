use warframe_macros::model;

/// The current time of day on Earth
#[model]
#[serde(rename_all = "lowercase")]
pub enum EarthState {
    /// Daytime
    Day,
    /// Nighttime
    Night,
}

/// The current Earth day/night cycle
#[model(endpoint = "/earthCycle", return_style = Object, timed)]
pub struct EarthCycle {
    /// Unique identifier for the cycle
    pub id: String,

    /// Whether it is currently day on Earth
    pub is_day: bool,

    /// The current state (day/night)
    pub state: EarthState,
}

#[cfg(test)]
mod test_earth_cycle {
    use rstest::rstest;
    use serde_json::from_str;

    use super::EarthCycle;
    use crate::worldstate::Queryable;

    type R = <EarthCycle as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/earth_cycle.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
