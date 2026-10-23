use warframe_macros::model;

/// Mission information for a Sentient Outpost
#[model]
pub struct SentientOutpostMission {
    /// The i18n of the node
    pub node: String,

    /// The i18n of the faction
    pub faction: String,

    /// The mission type
    #[serde(rename = "type")]
    pub mission_type: String,
}

/// Information about the current Sentient Outpost in Railjack
#[model(endpoint = "/sentientOutposts", return_style = Object, timed)]
pub struct SentientOutpost {
    /// Unique identifier for the outpost
    pub id: String,

    /// The mission details
    pub mission: Option<SentientOutpostMission>,

    /// Whether the outpost is currently active
    pub active: bool,
}

#[cfg(test)]
mod test_sentient_outpost {
    use rstest::rstest;
    use serde_json::from_str;

    use super::SentientOutpost;
    use crate::worldstate::Queryable;

    type R = <SentientOutpost as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/sentient_outpost.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
