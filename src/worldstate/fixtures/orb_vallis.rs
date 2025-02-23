use crate::fixture;

fixture! {
    orb_vallis,
r#"
{
  "id": "vallisCycle1740334020000",
  "expiry": "2025-02-23T18:27:08.000Z",
  "isWarm": false,
  "state": "cold",
  "activation": "2025-02-23T18:07:00.000Z",
  "timeLeft": "15m 7s",
  "shortString": "15m to Warm"
}
"#
---
r#"
{
  "id": "vallisCycle1740334020000",
  "expiry": "2025-02-23T18:27:08.000Z",
  "isWarm": false,
  "state": "cold",
  "activation": "2025-02-23T18:07:00.000Z",
  "timeLeft": "13m 37s",
  "shortString": "13m to Warm"
}
"#
}
