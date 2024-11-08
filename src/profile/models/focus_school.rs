use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum FocusSchool {
    #[serde(rename = "AP_ATTACK")]
    Madurai,
    #[serde(rename = "AP_DEFENSE")]
    Vazarin,
    #[serde(rename = "AP_TACTIC")]
    Naramon,
    #[serde(rename = "AP_WARD")]
    Unairu,
    #[serde(rename = "AP_POWER")]
    Zenurik,
}
