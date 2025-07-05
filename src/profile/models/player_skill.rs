use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerSkill {
    #[serde(rename = "LPP_SPACE")]
    Railjack,
    #[serde(rename = "LPS_GUNNERY")]
    RailjackGunnery,
    #[serde(rename = "LPS_TACTICAL")]
    RailjackTactical,
    #[serde(rename = "LPS_PILOTING")]
    RailjackPiloting,
    #[serde(rename = "LPS_ENGINEERING")]
    RailjackEngineering,
    #[serde(rename = "LPS_COMMAND")]
    RailjackCommand,
    #[serde(rename = "LPP_DRIFTER")]
    Drifter,
    #[serde(rename = "LPS_DRIFT_RIDING")]
    DrifterRiding,
    #[serde(rename = "LPS_DRIFT_COMBAT")]
    DrifterCombat,
    #[serde(rename = "LPS_DRIFT_OPPORTUNITY")]
    DrifterOpportunity,
    #[serde(rename = "LPS_DRIFT_ENDURANCE")]
    DrifterEndurance,
}
