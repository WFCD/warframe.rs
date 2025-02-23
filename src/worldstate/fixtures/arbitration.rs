use crate::fixture;

fixture! {
    arbitration,
r#"
{
  "node": "SolNode000",
  "nodeKey": "SolNode000",
  "activation": "1970-01-01T00:00:00.000Z",
  "expiry": "+275760-09-13T00:00:00.000Z",
  "enemy": "Tenno",
  "type": "Unknown",
  "typeKey": "Unknown",
  "archwing": false,
  "sharkwing": false
}
"#
---
r#"
{
  "node": "SolNode000",
  "nodeKey": "SolNode000",
  "activation": "1970-01-01T00:00:00.000Z",
  "expiry": "+275760-09-13T00:00:00.000Z",
  "enemy": "Tenno",
  "type": "Unknown",
  "typeKey": "Unknown",
  "archwing": false,
  "sharkwing": false
}
"#
}
