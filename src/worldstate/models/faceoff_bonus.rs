use chrono::{
    DateTime,
    Utc,
};
use warframe_macros::model;

/// The current Faceoff bonus window
#[model(endpoint = "/faceoffBonus", return_style = Object, timed)]
pub struct FaceoffBonus {
    /// When the next bonus window starts
    pub next: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod test_faceoff_bonus {
    use rstest::rstest;
    use serde_json::from_str;

    use super::FaceoffBonus;
    use crate::worldstate::Queryable;

    type R = <FaceoffBonus as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/faceoff_bonus.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
