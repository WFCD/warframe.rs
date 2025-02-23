use crate::fixture;

fixture! {
    global_upgrade,
r#"
[
  {
    "activation": "2025-02-21T19:00:00.000Z",
    "start": "2025-02-21T19:00:00.000Z",
    "expiry": "2025-02-24T19:00:00.000Z",
    "end": "2025-02-24T19:00:00.000Z",
    "upgrade": "Mission Kill XP",
    "operation": "is multiplied by",
    "operationSymbol": "x",
    "upgradeOperationValue": 2,
    "expired": false,
    "eta": "1d 53m 9s",
    "desc": "2x Mission Kill XP for 1d 53m 9s"
  }
]
"#
---
r#"
[
  {
    "activation": "2025-02-21T19:00:00.000Z",
    "start": "2025-02-21T19:00:00.000Z",
    "expiry": "2025-02-24T19:00:00.000Z",
    "end": "2025-02-24T19:00:00.000Z",
    "upgrade": "任務擊殺經驗",
    "operation": "被乘以",
    "operationSymbol": "x",
    "upgradeOperationValue": 2,
    "expired": false,
    "eta": "1d 52m 29s",
    "desc": "2x 任務擊殺經驗 for 1d 52m 29s"
  }
]
"#
}
