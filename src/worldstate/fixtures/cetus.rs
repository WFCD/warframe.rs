use crate::fixture;

fixture! {
    cetus,
r#"
{
  "id": "cetusCycle1740336060000",
  "expiry": "2025-02-23T18:41:00.000Z",
  "activation": "2025-02-23T17:01:00.000Z",
  "isDay": true,
  "state": "day",
  "timeLeft": "51m 20s",
  "isCetus": true,
  "shortString": "51m to Night"
}
"#
---
r#"
{
  "id": "cetusCycle1740336060000",
  "expiry": "2025-02-23T18:41:00.000Z",
  "activation": "2025-02-23T17:01:00.000Z",
  "isDay": true,
  "state": "day",
  "timeLeft": "51m 0s",
  "isCetus": true,
  "shortString": "51m to Night"
}
"#
}
