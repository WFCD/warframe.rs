use crate::fixture;

fixture! {
    event,
r#"
[
  {
    "id": "67a4dcce2a198564d62e1647",
    "activation": "2025-02-06T16:00:00.000Z",
    "startString": "-17d 1h 32m 50s",
    "expiry": "2025-03-05T16:00:00.000Z",
    "active": true,
    "maximumScore": 0,
    "currentScore": 0,
    "smallInterval": null,
    "largeInterval": null,
    "description": "Star Days",
    "tooltip": "Star Days",
    "node": "Fortuna (Venus)",
    "concurrentNodes": [],
    "rewards": [],
    "expired": false,
    "jobs": [],
    "previousJobs": [],
    "interimSteps": [],
    "progressSteps": [],
    "isPersonal": true,
    "regionDrops": [],
    "archwingDrops": [],
    "asString": "Star Days\nBattle on Fortuna (Venus)",
    "metadata": {

    },
    "completionBonuses": [],
    "altExpiry": "1970-01-01T00:00:00.000Z",
    "altActivation": "1970-01-01T00:00:00.000Z",
    "nextAlt": {
      "expiry": "1970-01-01T00:00:00.000Z",
      "activation": "1970-01-01T00:00:00.000Z"
    },
    "tag": "FortunaValentines"
  },
  {
    "id": "67a5035c2a198564d62e165e",
    "activation": "2025-02-06T19:00:00.000Z",
    "startString": "-16d 22h 32m 50s",
    "expiry": "2025-03-03T19:00:00.000Z",
    "active": true,
    "maximumScore": 0,
    "currentScore": 100,
    "smallInterval": null,
    "largeInterval": null,
    "faction": "Man in the Wall",
    "description": "Operation: Belly of the Beast",
    "tooltip": "Stop Parvos Granum! Collect Volatile Motes from Operation Ascension missions on Brutus (Uranus) and special Operation Alerts.\nVisit Ordis on Larunda Relay (Mercury) to claim rewards.",
    "node": "Brutus (Uranus)",
    "concurrentNodes": [],
    "scoreLocTag": "Volatile Motes Collection Progress",
    "rewards": [],
    "expired": false,
    "health": 100,
    "jobs": [],
    "previousJobs": [],
    "interimSteps": [],
    "progressSteps": [],
    "isPersonal": true,
    "isCommunity": true,
    "regionDrops": [],
    "archwingDrops": [],
    "asString": "Operation: Belly of the Beast : Man in the Wall\nBattle on Brutus (Uranus)\n100% Remaining",
    "metadata": {

    },
    "completionBonuses": [],
    "altExpiry": "1970-01-01T00:00:00.000Z",
    "altActivation": "1970-01-01T00:00:00.000Z",
    "nextAlt": {
      "expiry": "1970-01-01T00:00:00.000Z",
      "activation": "1970-01-01T00:00:00.000Z"
    },
    "tag": "JadeShadowsEvent"
  }
]
"#
---
r#"
[
  {
    "id": "67a4dcce2a198564d62e1647",
    "activation": "2025-02-06T16:00:00.000Z",
    "startString": "-17d 1h 57m 40s",
    "expiry": "2025-03-05T16:00:00.000Z",
    "active": true,
    "maximumScore": 0,
    "currentScore": 0,
    "smallInterval": null,
    "largeInterval": null,
    "description": "Valentines Fortuna Name",
    "tooltip": "Valentines Fortuna Name",
    "node": "SolarisUnitedHub1",
    "concurrentNodes": [],
    "rewards": [],
    "expired": false,
    "jobs": [],
    "previousJobs": [],
    "interimSteps": [],
    "progressSteps": [],
    "isPersonal": true,
    "regionDrops": [],
    "archwingDrops": [],
    "asString": "Valentines Fortuna Name\nBattle on SolarisUnitedHub1",
    "metadata": {

    },
    "completionBonuses": [],
    "altExpiry": "1970-01-01T00:00:00.000Z",
    "altActivation": "1970-01-01T00:00:00.000Z",
    "nextAlt": {
      "expiry": "1970-01-01T00:00:00.000Z",
      "activation": "1970-01-01T00:00:00.000Z"
    },
    "tag": "FortunaValentines"
  },
  {
    "id": "67a5035c2a198564d62e165e",
    "activation": "2025-02-06T19:00:00.000Z",
    "startString": "-16d 22h 57m 40s",
    "expiry": "2025-03-03T19:00:00.000Z",
    "active": true,
    "maximumScore": 0,
    "currentScore": 100,
    "smallInterval": null,
    "largeInterval": null,
    "faction": "Man in the Wall",
    "description": "Operation: Belly of the Beast",
    "tooltip": "Stop Parvos Granum! Collect Volatile Motes from Operation Ascension missions on Brutus (Uranus) and special Operation Alerts.\nVisit Ordis on Larunda Relay (Mercury) to claim rewards.",
    "node": "Brutus (Uranus)",
    "concurrentNodes": [],
    "scoreLocTag": "Volatile Motes Collection Progress",
    "rewards": [],
    "expired": false,
    "health": 100,
    "jobs": [],
    "previousJobs": [],
    "interimSteps": [],
    "progressSteps": [],
    "isPersonal": true,
    "isCommunity": true,
    "regionDrops": [],
    "archwingDrops": [],
    "asString": "Operation: Belly of the Beast : Man in the Wall\nBattle on Brutus (Uranus)\n100% Remaining",
    "metadata": {

    },
    "completionBonuses": [],
    "altExpiry": "1970-01-01T00:00:00.000Z",
    "altActivation": "1970-01-01T00:00:00.000Z",
    "nextAlt": {
      "expiry": "1970-01-01T00:00:00.000Z",
      "activation": "1970-01-01T00:00:00.000Z"
    },
    "tag": "JadeShadowsEvent"
  }
]
"#
}
