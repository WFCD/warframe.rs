use warframe_macros::model;

/// Represents the current state on cetus
#[model]
#[serde(rename_all = "lowercase")]
pub enum CetusState {
    /// Represents Cetus' day state
    Day,
    /// Represents Cetus' night state
    Night,
}

/// The Information about cetus
#[model(endpoint = "/cetusCycle", return_style = Object, timed)]
pub struct Cetus {
    /// The id of the cycle
    pub id: String,

    /// The state of Cetus (day/night)
    pub state: CetusState,
}

#[cfg(test)]
mod test_cetus {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Cetus;
    use crate::worldstate::{
        Queryable,
        fixtures::cetus::{
            cetus,
            cetus_en,
        },
    };

    type R = <Cetus as Queryable>::Return;

    #[rstest]
    fn test(cetus_en: &str) {
        from_str::<R>(cetus_en).unwrap();
    }

    #[rstest]
    fn test_ml(cetus: &str) {
        from_str::<R>(cetus).unwrap();
    }
}
