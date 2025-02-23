use crate::fixture;

fixture! {
    cambion_drift,
r#"
{
  "id": "cambionCycle1740336060000",
  "activation": "2025-02-23T17:01:00.000Z",
  "expiry": "2025-02-23T18:41:00.000Z",
  "timeLeft": "59m 10s",
  "state": "fass",
  "active": "fass"
}
"#
---
r#"
{
    "id": "cambionCycle1740336060000",
    "activation": "2025-02-23T17:01:00.000Z",
    "expiry": "2025-02-23T18:41:00.000Z",
    "timeLeft": "57m 10s",
    "state": "fass",
    "active": "fass"
}
"#
}
