use crate::fixture;

fixture! {
    alert,
r#"
[
  {
    "id": "677d45a494ad716c90006b9a",
    "activation": "2025-01-24T16:00:00.000Z",
    "startString": "-1h 36m 0s",
    "expiry": "2025-01-27T16:00:00.000Z",
    "active": true,
    "mission": {
      "node": "Laomedeia (Neptune)",
      "nodeKey": "Laomedeia (Neptune)",
      "type": "Disruption",
      "typeKey": "Disruption",
      "faction": "Corpus",
      "factionKey": "Corpus",
      "reward": {
        "items": [
          "Hollow Point"
        ],
        "countedItems": [],
        "credits": 30000,
        "asString": "Hollow Point + 30000cr",
        "itemString": "Hollow Point",
        "thumbnail": "",
        "color": 5198940
      },
      "minEnemyLevel": 30,
      "maxEnemyLevel": 35,
      "nightmare": false,
      "archwingRequired": false,
      "isSharkwing": false,
      "levelOverride": "Corpus Ship Disruption",
      "enemySpec": "Corpus Ship Survival A",
      "advancedSpawners": [],
      "requiredItems": [],
      "levelAuras": []
    },
    "eta": "2d 22h 23m 59s",
    "rewardTypes": [
      "other"
    ]
  },
  {
    "id": "677d4434074e34d5b9069c71",
    "activation": "2025-01-15T19:00:00.000Z",
    "startString": "-8d 22h 36m 0s",
    "expiry": "2025-02-15T19:00:00.000Z",
    "active": true,
    "mission": {
      "node": "Tharsis (Mars)",
      "nodeKey": "Tharsis (Mars)",
      "type": "Mobile Defense",
      "typeKey": "Mobile Defense",
      "faction": "Grineer",
      "factionKey": "Grineer",
      "reward": {
        "items": [
          "Giving Snake Glyph"
        ],
        "countedItems": [],
        "credits": 10000,
        "asString": "Giving Snake Glyph + 10000cr",
        "itemString": "Giving Snake Glyph",
        "thumbnail": "",
        "color": 5198940
      },
      "minEnemyLevel": 10,
      "maxEnemyLevel": 15,
      "nightmare": false,
      "archwingRequired": false,
      "isSharkwing": false,
      "levelOverride": "Grineer Settlement Mobile Defense",
      "enemySpec": "Desert Grineer Squad One",
      "advancedSpawners": [],
      "requiredItems": [],
      "levelAuras": []
    },
    "eta": "22d 1h 23m 59s",
    "rewardTypes": [
      "other"
    ]
  }
]
"#
---
r#"
[
  {
    "id": "677d45a494ad716c90006b9a",
    "activation": "2025-01-24T16:00:00.000Z",
    "startString": "-1h 50m 50s",
    "expiry": "2025-01-27T16:00:00.000Z",
    "active": true,
    "mission": {
      "node": "Laomedeia (海王星)",
      "nodeKey": "Laomedeia (Neptune)",
      "type": "中断",
      "typeKey": "Disruption",
      "faction": "Corpus",
      "factionKey": "Corpus",
      "reward": {
        "items": [
          "Hollow Point"
        ],
        "countedItems": [],
        "credits": 30000,
        "asString": "Hollow Point + 30000cr",
        "itemString": "Hollow Point",
        "thumbnail": "",
        "color": 5198940
      },
      "minEnemyLevel": 30,
      "maxEnemyLevel": 35,
      "nightmare": false,
      "archwingRequired": false,
      "isSharkwing": false,
      "levelOverride": "Corpus Ship Disruption",
      "enemySpec": "Corpus Ship Survival A",
      "advancedSpawners": [],
      "requiredItems": [],
      "levelAuras": []
    },
    "eta": "2d 22h 9m 9s",
    "rewardTypes": [
      "other"
    ]
  },
  {
    "id": "677d4434074e34d5b9069c71",
    "activation": "2025-01-15T19:00:00.000Z",
    "startString": "-8d 22h 50m 50s",
    "expiry": "2025-02-15T19:00:00.000Z",
    "active": true,
    "mission": {
      "node": "Tharsis (火星)",
      "nodeKey": "Tharsis (Mars)",
      "type": "移動防禦",
      "typeKey": "Mobile Defense",
      "faction": "Grineer",
      "factionKey": "Grineer",
      "reward": {
        "items": [
          "Giving Snake Glyph"
        ],
        "countedItems": [],
        "credits": 10000,
        "asString": "Giving Snake Glyph + 10000cr",
        "itemString": "Giving Snake Glyph",
        "thumbnail": "",
        "color": 5198940
      },
      "minEnemyLevel": 10,
      "maxEnemyLevel": 15,
      "nightmare": false,
      "archwingRequired": false,
      "isSharkwing": false,
      "levelOverride": "Grineer Settlement Mobile Defense",
      "enemySpec": "Desert Grineer Squad One",
      "advancedSpawners": [],
      "requiredItems": [],
      "levelAuras": []
    },
    "eta": "22d 1h 9m 9s",
    "rewardTypes": [
      "other"
    ]
  }
]
"#
}
