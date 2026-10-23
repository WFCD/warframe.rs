use warframe_macros::model;

use super::faction::{
    EnemyFaction,
    Faction,
};

/// Which faction controls the Zariman
#[model]
#[serde(rename_all = "lowercase")]
pub enum ZarimanFaction {
    /// Corpus are in control
    Corpus,
    /// Grineer are in control
    Grineer,
}

impl From<ZarimanFaction> for EnemyFaction {
    fn from(zf: ZarimanFaction) -> Self {
        match zf {
            ZarimanFaction::Corpus => EnemyFaction::Corpus,
            ZarimanFaction::Grineer => EnemyFaction::Grineer,
        }
    }
}

impl From<ZarimanFaction> for Faction {
    fn from(zf: ZarimanFaction) -> Self {
        Faction::Enemy(zf.into())
    }
}

/// The current Zariman cycle (Corpus/Grineer control)
#[model(endpoint = "/zarimanCycle", return_style = Object, timed)]
pub struct ZarimanCycle {
    /// Unique identifier for the cycle
    pub id: String,

    /// The current controlling faction
    pub state: ZarimanFaction,
}

#[cfg(test)]
mod test_zariman_cycle {
    use rstest::rstest;
    use serde_json::from_str;

    use super::ZarimanCycle;
    use crate::worldstate::Queryable;

    type R = <ZarimanCycle as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/*/zariman_cycle.json")]
        #[mode = str]
        fixture: &str,
    ) {
        let parsed = from_str::<R>(fixture).unwrap();
        println!("{parsed:#?}");
    }
}
