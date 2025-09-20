use warframe_macros::model;

use super::base::deserialize_f32_from_string;

/// Construction percentages for showing how far constructed the enemy fleets are
#[model(endpoint = "/constructionProgress", return_style = Object)]
pub struct ConstructionProgress {
    /// The progress of the Fomorian
    #[serde(deserialize_with = "deserialize_f32_from_string")]
    pub fomorian_progress: f32,

    /// The progress of the Razorback
    #[serde(deserialize_with = "deserialize_f32_from_string")]
    pub razorback_progress: f32,

    /// No clue what this is tbh
    #[serde(deserialize_with = "deserialize_f32_from_string")]
    pub unknown_progress: f32,
}

#[cfg(test)]
mod test_construction_progress {
    use rstest::rstest;
    use serde_json::from_str;

    use super::ConstructionProgress;
    use crate::worldstate::Queryable;

    type R = <ConstructionProgress as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/construction_progress.json")]
        #[mode = str]
        construction_progress_en: &str,
    ) {
        from_str::<R>(construction_progress_en).unwrap();
    }
}
