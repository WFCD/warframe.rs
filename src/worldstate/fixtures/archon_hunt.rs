use crate::fixture;

fixture! {
    archon_hunt,
r#"
{
  "id": "678d8e7e92bac20351c1c18d",
  "activation": "2025-01-20T00:00:00.000Z",
  "startString": "-4d 18h 22m 0s",
  "expiry": "2025-01-27T00:00:00.000Z",
  "active": true,
  "rewardPool": "Archon Sortie Rewards",
  "variants": [],
  "missions": [
    {
      "node": "Eurasia (Earth)",
      "nodeKey": "Eurasia (Earth)",
      "type": "Spy",
      "typeKey": "Spy",
      "nightmare": false,
      "archwingRequired": false,
      "isSharkwing": false,
      "advancedSpawners": [],
      "requiredItems": [],
      "levelAuras": []
    },
    {
      "node": "Everest (Earth)",
      "nodeKey": "Everest (Earth)",
      "type": "Interception",
      "typeKey": "Interception",
      "nightmare": false,
      "archwingRequired": false,
      "isSharkwing": false,
      "advancedSpawners": [],
      "requiredItems": [],
      "levelAuras": []
    },
    {
      "node": "Oro (Earth)",
      "nodeKey": "Oro (Earth)",
      "type": "Assassination",
      "typeKey": "Assassination",
      "nightmare": false,
      "archwingRequired": false,
      "isSharkwing": false,
      "advancedSpawners": [],
      "requiredItems": [],
      "levelAuras": []
    }
  ],
  "boss": "Archon Boreal",
  "faction": "Narmer",
  "factionKey": "Narmer",
  "expired": false,
  "eta": "2d 5h 37m 59s"
}
"#
---
r#"
{
  "id": "678d8e7e92bac20351c1c18d",
  "activation": "2025-01-20T00:00:00.000Z",
  "startString": "-4d 18h 21m 20s",
  "expiry": "2025-01-27T00:00:00.000Z",
  "active": true,
  "rewardPool": "Archon Sortie Rewards",
  "variants": [],
  "missions": [
    {
      "node": "Eurasia (地球)",
      "nodeKey": "Eurasia (Earth)",
      "type": "間諜",
      "typeKey": "Spy",
      "nightmare": false,
      "archwingRequired": false,
      "isSharkwing": false,
      "advancedSpawners": [],
      "requiredItems": [],
      "levelAuras": []
    },
    {
      "node": "Everest (地球)",
      "nodeKey": "Everest (Earth)",
      "type": "攔截",
      "typeKey": "Interception",
      "nightmare": false,
      "archwingRequired": false,
      "isSharkwing": false,
      "advancedSpawners": [],
      "requiredItems": [],
      "levelAuras": []
    },
    {
      "node": "Oro (地球)",
      "nodeKey": "Oro (Earth)",
      "type": "刺殺",
      "typeKey": "Assassination",
      "nightmare": false,
      "archwingRequired": false,
      "isSharkwing": false,
      "advancedSpawners": [],
      "requiredItems": [],
      "levelAuras": []
    }
  ],
  "boss": "诡文枭主",
  "faction": "合一众",
  "factionKey": "Narmer",
  "expired": false,
  "eta": "2d 5h 38m 39s"
}
"#
}
