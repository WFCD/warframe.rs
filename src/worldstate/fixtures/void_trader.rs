use crate::fixture;

fixture! {
    void_trader,
r#"
{
  "id": "5d1e07a0a38e4a4fdd7cefca",
  "activation": "2025-03-07T14:00:00.000Z",
  "startString": "11d 19h 41m 49s",
  "expiry": "2025-03-09T14:00:00.000Z",
  "active": false,
  "character": "Baro Ki'Teer",
  "location": "Strata Relay (Earth)",
  "inventory": [],
  "psId": "5d1e07a0a38e4a4fdd7cefca0",
  "endString": "13d 19h 41m 49s",
  "initialStart": "1970-01-01T00:00:00.000Z",
  "schedule": []
}
"#
---
r#"
{
  "id": "5d1e07a0a38e4a4fdd7cefca",
  "activation": "2025-03-07T14:00:00.000Z",
  "startString": "11d 19h 40m 29s",
  "expiry": "2025-03-09T14:00:00.000Z",
  "active": false,
  "character": "Baro Ki'Teer",
  "location": "Strata 中继站 (地球)",
  "inventory": [],
  "psId": "5d1e07a0a38e4a4fdd7cefca0",
  "endString": "13d 19h 40m 29s",
  "initialStart": "1970-01-01T00:00:00.000Z",
  "schedule": []
}
"#
}
