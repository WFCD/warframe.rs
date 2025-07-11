use crate::fixture;

fixture! {
    fissure,
r#"
[
  {
    "id": "67bb53227acd989e43c1c18d",
    "activation": "2025-02-23T16:56:02.289Z",
    "startString": "-1h 1m 57s",
    "expiry": "2025-02-23T17:57:30.081Z",
    "active": false,
    "node": "Alator (Mars)",
    "missionType": "Interception",
    "missionKey": "Interception",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Alator (Mars)",
    "tier": "Lith",
    "tierNum": 1,
    "expired": true,
    "eta": "-30s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb53227acd989e43c1c18e",
    "activation": "2025-02-23T16:56:02.289Z",
    "startString": "-1h 1m 57s",
    "expiry": "2025-02-23T18:22:14.741Z",
    "active": true,
    "node": "Eurasia (Earth)",
    "missionType": "Mobile Defense",
    "missionKey": "Mobile Defense",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Eurasia (Earth)",
    "tier": "Lith",
    "tierNum": 1,
    "expired": false,
    "eta": "24m 14s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb553eb776075ffec1c18d",
    "activation": "2025-02-23T17:05:02.394Z",
    "startString": "-52m 57s",
    "expiry": "2025-02-23T18:32:54.362Z",
    "active": true,
    "node": "Tamu (Kuva Fortress)",
    "missionType": "Disruption",
    "missionKey": "Disruption",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Tamu (Kuva Fortress)",
    "tier": "Requiem",
    "tierNum": 5,
    "expired": false,
    "eta": "34m 54s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb55b68ade77472ac1c18d",
    "activation": "2025-02-23T17:07:02.747Z",
    "startString": "-50m 57s",
    "expiry": "2025-02-23T18:44:24.169Z",
    "active": true,
    "node": "Circulus (Lua)",
    "missionType": "Survival",
    "missionKey": "Survival",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Circulus (Lua)",
    "tier": "Omnia",
    "tierNum": 6,
    "expired": false,
    "eta": "46m 24s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb571f627a78a239c1c18d",
    "activation": "2025-02-23T17:13:03.091Z",
    "startString": "-44m 57s",
    "expiry": "2025-02-23T18:14:46.446Z",
    "active": true,
    "node": "Nabuk (Kuva Fortress)",
    "missionType": "Defense",
    "missionKey": "Defense",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Nabuk (Kuva Fortress)",
    "tier": "Requiem",
    "tierNum": 5,
    "expired": false,
    "eta": "16m 46s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb5976782b30dad1c1c18d",
    "activation": "2025-02-23T17:23:02.811Z",
    "startString": "-34m 57s",
    "expiry": "2025-02-23T19:02:33.979Z",
    "active": true,
    "node": "Draco (Ceres)",
    "missionType": "Survival",
    "missionKey": "Survival",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Draco (Ceres)",
    "tier": "Meso",
    "tierNum": 2,
    "expired": false,
    "eta": "1h 4m 33s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb5aa2d93475da36c1c18d",
    "activation": "2025-02-23T17:28:02.848Z",
    "startString": "-29m 57s",
    "expiry": "2025-02-23T18:52:32.197Z",
    "active": true,
    "node": "Berehynia (Sedna)",
    "missionType": "Interception",
    "missionKey": "Interception",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Berehynia (Sedna)",
    "tier": "Axi",
    "tierNum": 4,
    "expired": false,
    "eta": "54m 32s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb5aa2d93475da36c1c18e",
    "activation": "2025-02-23T17:28:02.848Z",
    "startString": "-29m 57s",
    "expiry": "2025-02-23T19:03:50.114Z",
    "active": true,
    "node": "Xini (Eris)",
    "missionType": "Interception",
    "missionKey": "Interception",
    "enemy": "Infested",
    "enemyKey": "Infested",
    "nodeKey": "Xini (Eris)",
    "tier": "Axi",
    "tierNum": 4,
    "expired": false,
    "eta": "1h 5m 49s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb5aa2d93475da36c1c18f",
    "activation": "2025-02-23T17:28:02.848Z",
    "startString": "-29m 57s",
    "expiry": "2025-02-23T19:08:28.878Z",
    "active": true,
    "node": "Morax (Europa)",
    "missionType": "Mobile Defense",
    "missionKey": "Mobile Defense",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Morax (Europa)",
    "tier": "Neo",
    "tierNum": 3,
    "expired": false,
    "eta": "1h 10m 28s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb5aa2d93475da36c1c190",
    "activation": "2025-02-23T17:28:02.848Z",
    "startString": "-29m 57s",
    "expiry": "2025-02-23T19:02:10.393Z",
    "active": true,
    "node": "Ani (Void)",
    "missionType": "Survival",
    "missionKey": "Survival",
    "enemy": "Orokin",
    "enemyKey": "Orokin",
    "nodeKey": "Ani (Void)",
    "tier": "Neo",
    "tierNum": 3,
    "expired": false,
    "eta": "1h 4m 10s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb5adfbd7e373e21c1c18d",
    "activation": "2025-02-23T17:29:03.138Z",
    "startString": "-28m 56s",
    "expiry": "2025-02-23T19:05:40.806Z",
    "active": true,
    "node": "Tuvul Commons (Zariman)",
    "missionType": "Void Cascade",
    "missionKey": "Void Cascade",
    "enemy": "Crossfire",
    "enemyKey": "Crossfire",
    "nodeKey": "Tuvul Commons (Zariman)",
    "tier": "Omnia",
    "tierNum": 6,
    "expired": false,
    "eta": "1h 7m 40s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5adfbd7e373e21c1c18e",
    "activation": "2025-02-23T17:29:03.138Z",
    "startString": "-28m 56s",
    "expiry": "2025-02-23T18:32:34.109Z",
    "active": true,
    "node": "Cambire (Deimos)",
    "missionType": "Alchemy",
    "missionKey": "Alchemy",
    "enemy": "The Murmur",
    "enemyKey": "The Murmur",
    "nodeKey": "Cambire (Deimos)",
    "tier": "Omnia",
    "tierNum": 6,
    "expired": false,
    "eta": "34m 33s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5b56eec1a02832c1c18d",
    "activation": "2025-02-23T17:31:02.301Z",
    "startString": "-26m 57s",
    "expiry": "2025-02-23T19:06:32.775Z",
    "active": true,
    "node": "Selkie (Sedna)",
    "missionType": "Survival",
    "missionKey": "Survival",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Selkie (Sedna)",
    "tier": "Axi",
    "tierNum": 4,
    "expired": false,
    "eta": "1h 8m 32s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5b56eec1a02832c1c18e",
    "activation": "2025-02-23T17:31:02.301Z",
    "startString": "-26m 57s",
    "expiry": "2025-02-23T19:17:18.227Z",
    "active": true,
    "node": "Aten (Void)",
    "missionType": "Mobile Defense",
    "missionKey": "Mobile Defense",
    "enemy": "Orokin",
    "enemyKey": "Orokin",
    "nodeKey": "Aten (Void)",
    "tier": "Axi",
    "tierNum": 4,
    "expired": false,
    "eta": "1h 19m 18s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5c0a5d18322abfc1c18d",
    "activation": "2025-02-23T17:34:02.958Z",
    "startString": "-23m 57s",
    "expiry": "2025-02-23T18:41:45.681Z",
    "active": true,
    "node": "Callisto (Jupiter)",
    "missionType": "Interception",
    "missionKey": "Interception",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Callisto (Jupiter)",
    "tier": "Meso",
    "tierNum": 2,
    "expired": false,
    "eta": "43m 45s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5c0a5d18322abfc1c18e",
    "activation": "2025-02-23T17:34:02.958Z",
    "startString": "-23m 57s",
    "expiry": "2025-02-23T19:05:17.047Z",
    "active": true,
    "node": "Sharpless (Phobos)",
    "missionType": "Mobile Defense",
    "missionKey": "Mobile Defense",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Sharpless (Phobos)",
    "tier": "Meso",
    "tierNum": 2,
    "expired": false,
    "eta": "1h 7m 16s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5c0a5d18322abfc1c18f",
    "activation": "2025-02-23T17:34:02.958Z",
    "startString": "-23m 57s",
    "expiry": "2025-02-23T19:00:49.285Z",
    "active": true,
    "node": "Casta (Ceres)",
    "missionType": "Defense",
    "missionKey": "Defense",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Casta (Ceres)",
    "tier": "Meso",
    "tierNum": 2,
    "expired": false,
    "eta": "1h 2m 49s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5cbed9786ca120c1c18d",
    "activation": "2025-02-23T17:37:02.872Z",
    "startString": "-20m 57s",
    "expiry": "2025-02-23T19:34:34.019Z",
    "active": true,
    "node": "Stephano (Uranus)",
    "missionType": "Defense",
    "missionKey": "Defense",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Stephano (Uranus)",
    "tier": "Neo",
    "tierNum": 3,
    "expired": false,
    "eta": "1h 36m 33s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5cbed9786ca120c1c18e",
    "activation": "2025-02-23T17:37:02.872Z",
    "startString": "-20m 57s",
    "expiry": "2025-02-23T18:56:18.160Z",
    "active": true,
    "node": "Valefor (Europa)",
    "missionType": "Excavation",
    "missionKey": "Excavation",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Valefor (Europa)",
    "tier": "Neo",
    "tierNum": 3,
    "expired": false,
    "eta": "58m 18s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5cbed9786ca120c1c18f",
    "activation": "2025-02-23T17:37:02.872Z",
    "startString": "-20m 57s",
    "expiry": "2025-02-23T19:01:53.830Z",
    "active": true,
    "node": "Neso (Neptune)",
    "missionType": "Extermination",
    "missionKey": "Extermination",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Neso (Neptune)",
    "tier": "Neo",
    "tierNum": 3,
    "expired": false,
    "eta": "1h 3m 53s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5dafebca32475dc1c18d",
    "activation": "2025-02-23T17:41:03.113Z",
    "startString": "-16m 57s",
    "expiry": "2025-02-23T18:53:05.220Z",
    "active": true,
    "node": "Kiliken (Venus)",
    "missionType": "Excavation",
    "missionKey": "Excavation",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Kiliken (Venus)",
    "tier": "Lith",
    "tierNum": 1,
    "expired": false,
    "eta": "55m 5s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb4151c262f06e94c1c18c",
    "activation": "2025-02-23T16:40:01.396Z",
    "startString": "-1h 17m 58s",
    "expiry": "2025-02-23T18:10:01.396Z",
    "active": true,
    "node": "Korm's Belt (Earth)",
    "missionType": "Skirmish",
    "missionKey": "Skirmish",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Korm's Belt (Earth)",
    "tier": "Lith",
    "tierNum": 1,
    "expired": false,
    "eta": "12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4151c262f06e94c1c18d",
    "activation": "2025-02-23T16:40:01.400Z",
    "startString": "-1h 17m 58s",
    "expiry": "2025-02-23T18:10:01.400Z",
    "active": true,
    "node": "Orvin-Haarc (Venus)",
    "missionType": "Spy",
    "missionKey": "Spy",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Orvin-Haarc (Venus)",
    "tier": "Lith",
    "tierNum": 1,
    "expired": false,
    "eta": "12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4151c262f06e94c1c18e",
    "activation": "2025-02-23T16:40:01.401Z",
    "startString": "-1h 17m 58s",
    "expiry": "2025-02-23T18:10:01.401Z",
    "active": true,
    "node": "Mordo Cluster (Saturn)",
    "missionType": "Skirmish",
    "missionKey": "Skirmish",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Mordo Cluster (Saturn)",
    "tier": "Meso",
    "tierNum": 2,
    "expired": false,
    "eta": "12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4151c262f06e94c1c18f",
    "activation": "2025-02-23T16:40:01.402Z",
    "startString": "-1h 17m 58s",
    "expiry": "2025-02-23T18:10:01.402Z",
    "active": true,
    "node": "Sovereign Grasp (Neptune)",
    "missionType": "Volatile",
    "missionKey": "Volatile",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Sovereign Grasp (Neptune)",
    "tier": "Neo",
    "tierNum": 3,
    "expired": false,
    "eta": "12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4151c262f06e94c1c191",
    "activation": "2025-02-23T16:40:01.403Z",
    "startString": "-1h 17m 58s",
    "expiry": "2025-02-23T18:10:01.403Z",
    "active": true,
    "node": "Numina (Veil)",
    "missionType": "Volatile",
    "missionKey": "Volatile",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Numina (Veil)",
    "tier": "Axi",
    "tierNum": 4,
    "expired": false,
    "eta": "12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4151c262f06e94c1c190",
    "activation": "2025-02-23T16:40:01.403Z",
    "startString": "-1h 17m 58s",
    "expiry": "2025-02-23T18:10:01.403Z",
    "active": true,
    "node": "Profit Margin (Pluto)",
    "missionType": "Volatile",
    "missionKey": "Volatile",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Profit Margin (Pluto)",
    "tier": "Axi",
    "tierNum": 4,
    "expired": false,
    "eta": "12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4f61a69998d076c1c18c",
    "activation": "2025-02-23T17:40:01.396Z",
    "startString": "-17m 58s",
    "expiry": "2025-02-23T19:10:01.396Z",
    "active": true,
    "node": "Iota Temple (Earth)",
    "missionType": "Skirmish",
    "missionKey": "Skirmish",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Iota Temple (Earth)",
    "tier": "Lith",
    "tierNum": 1,
    "expired": false,
    "eta": "1h 12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4f61a69998d076c1c18d",
    "activation": "2025-02-23T17:40:01.400Z",
    "startString": "-17m 58s",
    "expiry": "2025-02-23T19:10:01.400Z",
    "active": true,
    "node": "Beacon Shield Ring (Venus)",
    "missionType": "Volatile",
    "missionKey": "Volatile",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Beacon Shield Ring (Venus)",
    "tier": "Lith",
    "tierNum": 1,
    "expired": false,
    "eta": "1h 12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4f61a69998d076c1c18f",
    "activation": "2025-02-23T17:40:01.402Z",
    "startString": "-17m 58s",
    "expiry": "2025-02-23T19:10:01.402Z",
    "active": true,
    "node": "Brom Cluster (Neptune)",
    "missionType": "Spy",
    "missionKey": "Spy",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Brom Cluster (Neptune)",
    "tier": "Neo",
    "tierNum": 3,
    "expired": false,
    "eta": "1h 12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4f61a69998d076c1c18e",
    "activation": "2025-02-23T17:40:01.401Z",
    "startString": "-17m 58s",
    "expiry": "2025-02-23T19:10:01.401Z",
    "active": true,
    "node": "Vand Cluster (Saturn)",
    "missionType": "Skirmish",
    "missionKey": "Skirmish",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Vand Cluster (Saturn)",
    "tier": "Meso",
    "tierNum": 2,
    "expired": false,
    "eta": "1h 12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4f61a69998d076c1c190",
    "activation": "2025-02-23T17:40:01.403Z",
    "startString": "-17m 58s",
    "expiry": "2025-02-23T19:10:01.403Z",
    "active": true,
    "node": "Peregrine Axis (Pluto)",
    "missionType": "Orphix",
    "missionKey": "Orphix",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Peregrine Axis (Pluto)",
    "tier": "Axi",
    "tierNum": 4,
    "expired": false,
    "eta": "1h 12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4f61a69998d076c1c191",
    "activation": "2025-02-23T17:40:01.403Z",
    "startString": "-17m 58s",
    "expiry": "2025-02-23T19:10:01.403Z",
    "active": true,
    "node": "Calabash (Veil)",
    "missionType": "Extermination",
    "missionKey": "Extermination",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Calabash (Veil)",
    "tier": "Axi",
    "tierNum": 4,
    "expired": false,
    "eta": "1h 12m 1s",
    "isStorm": true,
    "isHard": false
  }
]
"#
---
r#"
[
  {
    "id": "67bb53227acd989e43c1c18d",
    "activation": "2025-02-23T16:56:02.289Z",
    "startString": "-1h 1m 57s",
    "expiry": "2025-02-23T17:57:30.081Z",
    "active": false,
    "node": "Alator (火星)",
    "missionType": "攔截",
    "missionKey": "Interception",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Alator (Mars)",
    "tier": "古纪",
    "tierNum": 1,
    "expired": true,
    "eta": "-30s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb53227acd989e43c1c18e",
    "activation": "2025-02-23T16:56:02.289Z",
    "startString": "-1h 1m 57s",
    "expiry": "2025-02-23T18:22:14.741Z",
    "active": true,
    "node": "Eurasia (地球)",
    "missionType": "移動防禦",
    "missionKey": "Mobile Defense",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Eurasia (Earth)",
    "tier": "古纪",
    "tierNum": 1,
    "expired": false,
    "eta": "24m 14s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb553eb776075ffec1c18d",
    "activation": "2025-02-23T17:05:02.394Z",
    "startString": "-52m 57s",
    "expiry": "2025-02-23T18:32:54.362Z",
    "active": true,
    "node": "Tamu (Kuva 要塞)",
    "missionType": "中断",
    "missionKey": "Disruption",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Tamu (Kuva Fortress)",
    "tier": "安魂",
    "tierNum": 5,
    "expired": false,
    "eta": "34m 54s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb55b68ade77472ac1c18d",
    "activation": "2025-02-23T17:07:02.747Z",
    "startString": "-50m 57s",
    "expiry": "2025-02-23T18:44:24.169Z",
    "active": true,
    "node": "Circulus (月球)",
    "missionType": "生存",
    "missionKey": "Survival",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Circulus (Lua)",
    "tier": "全能",
    "tierNum": 6,
    "expired": false,
    "eta": "46m 24s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb571f627a78a239c1c18d",
    "activation": "2025-02-23T17:13:03.091Z",
    "startString": "-44m 56s",
    "expiry": "2025-02-23T18:14:46.446Z",
    "active": true,
    "node": "Nabuk (Kuva 要塞)",
    "missionType": "防禦",
    "missionKey": "Defense",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Nabuk (Kuva Fortress)",
    "tier": "安魂",
    "tierNum": 5,
    "expired": false,
    "eta": "16m 46s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb5976782b30dad1c1c18d",
    "activation": "2025-02-23T17:23:02.811Z",
    "startString": "-34m 57s",
    "expiry": "2025-02-23T19:02:33.979Z",
    "active": true,
    "node": "Draco (穀神星)",
    "missionType": "生存",
    "missionKey": "Survival",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Draco (Ceres)",
    "tier": "前纪",
    "tierNum": 2,
    "expired": false,
    "eta": "1h 4m 33s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb5aa2d93475da36c1c18d",
    "activation": "2025-02-23T17:28:02.848Z",
    "startString": "-29m 57s",
    "expiry": "2025-02-23T18:52:32.197Z",
    "active": true,
    "node": "Berehynia (賽德娜)",
    "missionType": "攔截",
    "missionKey": "Interception",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Berehynia (Sedna)",
    "tier": "后纪",
    "tierNum": 4,
    "expired": false,
    "eta": "54m 32s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb5aa2d93475da36c1c18e",
    "activation": "2025-02-23T17:28:02.848Z",
    "startString": "-29m 57s",
    "expiry": "2025-02-23T19:03:50.114Z",
    "active": true,
    "node": "Xini (鬩神星)",
    "missionType": "攔截",
    "missionKey": "Interception",
    "enemy": "Infested",
    "enemyKey": "Infested",
    "nodeKey": "Xini (Eris)",
    "tier": "后纪",
    "tierNum": 4,
    "expired": false,
    "eta": "1h 5m 50s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb5aa2d93475da36c1c18f",
    "activation": "2025-02-23T17:28:02.848Z",
    "startString": "-29m 57s",
    "expiry": "2025-02-23T19:08:28.878Z",
    "active": true,
    "node": "Morax (歐羅巴)",
    "missionType": "移動防禦",
    "missionKey": "Mobile Defense",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Morax (Europa)",
    "tier": "中纪",
    "tierNum": 3,
    "expired": false,
    "eta": "1h 10m 28s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb5aa2d93475da36c1c190",
    "activation": "2025-02-23T17:28:02.848Z",
    "startString": "-29m 57s",
    "expiry": "2025-02-23T19:02:10.393Z",
    "active": true,
    "node": "Ani (虚空)",
    "missionType": "生存",
    "missionKey": "Survival",
    "enemy": "奥罗金",
    "enemyKey": "Orokin",
    "nodeKey": "Ani (Void)",
    "tier": "中纪",
    "tierNum": 3,
    "expired": false,
    "eta": "1h 4m 10s",
    "isStorm": false,
    "isHard": true
  },
  {
    "id": "67bb5adfbd7e373e21c1c18d",
    "activation": "2025-02-23T17:29:03.138Z",
    "startString": "-28m 56s",
    "expiry": "2025-02-23T19:05:40.806Z",
    "active": true,
    "node": "涂沃主厅(扎里曼)",
    "missionType": "虚空覆涌",
    "missionKey": "Void Cascade",
    "enemy": "雙方交戰",
    "enemyKey": "Crossfire",
    "nodeKey": "Tuvul Commons (Zariman)",
    "tier": "全能",
    "tierNum": 6,
    "expired": false,
    "eta": "1h 7m 40s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5adfbd7e373e21c1c18e",
    "activation": "2025-02-23T17:29:03.138Z",
    "startString": "-28m 56s",
    "expiry": "2025-02-23T18:32:34.109Z",
    "active": true,
    "node": "异化区(火卫二)",
    "missionType": "Alchemy",
    "missionKey": "Alchemy",
    "enemy": "低语者",
    "enemyKey": "The Murmur",
    "nodeKey": "Cambire (Deimos)",
    "tier": "全能",
    "tierNum": 6,
    "expired": false,
    "eta": "34m 34s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5b56eec1a02832c1c18d",
    "activation": "2025-02-23T17:31:02.301Z",
    "startString": "-26m 57s",
    "expiry": "2025-02-23T19:06:32.775Z",
    "active": true,
    "node": "Selkie (賽德娜)",
    "missionType": "生存",
    "missionKey": "Survival",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Selkie (Sedna)",
    "tier": "后纪",
    "tierNum": 4,
    "expired": false,
    "eta": "1h 8m 32s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5b56eec1a02832c1c18e",
    "activation": "2025-02-23T17:31:02.301Z",
    "startString": "-26m 57s",
    "expiry": "2025-02-23T19:17:18.227Z",
    "active": true,
    "node": "Aten (虚空)",
    "missionType": "移動防禦",
    "missionKey": "Mobile Defense",
    "enemy": "奥罗金",
    "enemyKey": "Orokin",
    "nodeKey": "Aten (Void)",
    "tier": "后纪",
    "tierNum": 4,
    "expired": false,
    "eta": "1h 19m 18s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5c0a5d18322abfc1c18d",
    "activation": "2025-02-23T17:34:02.958Z",
    "startString": "-23m 57s",
    "expiry": "2025-02-23T18:41:45.681Z",
    "active": true,
    "node": "Callisto (木星)",
    "missionType": "攔截",
    "missionKey": "Interception",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Callisto (Jupiter)",
    "tier": "前纪",
    "tierNum": 2,
    "expired": false,
    "eta": "43m 45s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5c0a5d18322abfc1c18e",
    "activation": "2025-02-23T17:34:02.958Z",
    "startString": "-23m 57s",
    "expiry": "2025-02-23T19:05:17.047Z",
    "active": true,
    "node": "Sharpless (火衛一)",
    "missionType": "移動防禦",
    "missionKey": "Mobile Defense",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Sharpless (Phobos)",
    "tier": "前纪",
    "tierNum": 2,
    "expired": false,
    "eta": "1h 7m 16s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5c0a5d18322abfc1c18f",
    "activation": "2025-02-23T17:34:02.958Z",
    "startString": "-23m 57s",
    "expiry": "2025-02-23T19:00:49.285Z",
    "active": true,
    "node": "Casta (穀神星)",
    "missionType": "防禦",
    "missionKey": "Defense",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Casta (Ceres)",
    "tier": "前纪",
    "tierNum": 2,
    "expired": false,
    "eta": "1h 2m 49s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5cbed9786ca120c1c18d",
    "activation": "2025-02-23T17:37:02.872Z",
    "startString": "-20m 57s",
    "expiry": "2025-02-23T19:34:34.019Z",
    "active": true,
    "node": "Stephano (天王星)",
    "missionType": "防禦",
    "missionKey": "Defense",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Stephano (Uranus)",
    "tier": "中纪",
    "tierNum": 3,
    "expired": false,
    "eta": "1h 36m 33s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5cbed9786ca120c1c18e",
    "activation": "2025-02-23T17:37:02.872Z",
    "startString": "-20m 57s",
    "expiry": "2025-02-23T18:56:18.160Z",
    "active": true,
    "node": "Valefor (歐羅巴)",
    "missionType": "挖掘",
    "missionKey": "Excavation",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Valefor (Europa)",
    "tier": "中纪",
    "tierNum": 3,
    "expired": false,
    "eta": "58m 18s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5cbed9786ca120c1c18f",
    "activation": "2025-02-23T17:37:02.872Z",
    "startString": "-20m 57s",
    "expiry": "2025-02-23T19:01:53.830Z",
    "active": true,
    "node": "Neso (海王星)",
    "missionType": "殲滅",
    "missionKey": "Extermination",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Neso (Neptune)",
    "tier": "中纪",
    "tierNum": 3,
    "expired": false,
    "eta": "1h 3m 53s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb5dafebca32475dc1c18d",
    "activation": "2025-02-23T17:41:03.113Z",
    "startString": "-16m 56s",
    "expiry": "2025-02-23T18:53:05.220Z",
    "active": true,
    "node": "Kiliken (金星)",
    "missionType": "挖掘",
    "missionKey": "Excavation",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Kiliken (Venus)",
    "tier": "古纪",
    "tierNum": 1,
    "expired": false,
    "eta": "55m 5s",
    "isStorm": false,
    "isHard": false
  },
  {
    "id": "67bb4151c262f06e94c1c18c",
    "activation": "2025-02-23T16:40:01.396Z",
    "startString": "-1h 17m 58s",
    "expiry": "2025-02-23T18:10:01.396Z",
    "active": true,
    "node": "克姆地带 (地球比邻星域)",
    "missionType": "前哨战",
    "missionKey": "Skirmish",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Korm's Belt (Earth)",
    "tier": "古纪",
    "tierNum": 1,
    "expired": false,
    "eta": "12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4151c262f06e94c1c18d",
    "activation": "2025-02-23T16:40:01.400Z",
    "startString": "-1h 17m 58s",
    "expiry": "2025-02-23T18:10:01.400Z",
    "active": true,
    "node": "欧文－哈克 (金星比邻星域)",
    "missionType": "間諜",
    "missionKey": "Spy",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Orvin-Haarc (Venus)",
    "tier": "古纪",
    "tierNum": 1,
    "expired": false,
    "eta": "12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4151c262f06e94c1c18e",
    "activation": "2025-02-23T16:40:01.401Z",
    "startString": "-1h 17m 58s",
    "expiry": "2025-02-23T18:10:01.401Z",
    "active": true,
    "node": "魔多星团 (土星比邻星域)",
    "missionType": "前哨战",
    "missionKey": "Skirmish",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Mordo Cluster (Saturn)",
    "tier": "前纪",
    "tierNum": 2,
    "expired": false,
    "eta": "12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4151c262f06e94c1c18f",
    "activation": "2025-02-23T16:40:01.402Z",
    "startString": "-1h 17m 58s",
    "expiry": "2025-02-23T18:10:01.402Z",
    "active": true,
    "node": "星主之握 (海王星比邻星域)",
    "missionType": "爆发",
    "missionKey": "Volatile",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Sovereign Grasp (Neptune)",
    "tier": "中纪",
    "tierNum": 3,
    "expired": false,
    "eta": "12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4151c262f06e94c1c191",
    "activation": "2025-02-23T16:40:01.403Z",
    "startString": "-1h 17m 58s",
    "expiry": "2025-02-23T18:10:01.403Z",
    "active": true,
    "node": "努秘 (面纱)",
    "missionType": "爆发",
    "missionKey": "Volatile",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Numina (Veil)",
    "tier": "后纪",
    "tierNum": 4,
    "expired": false,
    "eta": "12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4151c262f06e94c1c190",
    "activation": "2025-02-23T16:40:01.403Z",
    "startString": "-1h 17m 58s",
    "expiry": "2025-02-23T18:10:01.403Z",
    "active": true,
    "node": "利益外缘 (冥王星比邻星域)",
    "missionType": "爆发",
    "missionKey": "Volatile",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Profit Margin (Pluto)",
    "tier": "后纪",
    "tierNum": 4,
    "expired": false,
    "eta": "12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4f61a69998d076c1c18c",
    "activation": "2025-02-23T17:40:01.396Z",
    "startString": "-17m 58s",
    "expiry": "2025-02-23T19:10:01.396Z",
    "active": true,
    "node": "虚无神殿 (地球比邻星域)",
    "missionType": "前哨战",
    "missionKey": "Skirmish",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Iota Temple (Earth)",
    "tier": "古纪",
    "tierNum": 1,
    "expired": false,
    "eta": "1h 12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4f61a69998d076c1c18d",
    "activation": "2025-02-23T17:40:01.400Z",
    "startString": "-17m 58s",
    "expiry": "2025-02-23T19:10:01.400Z",
    "active": true,
    "node": "卫标星环 (金星比邻星域)",
    "missionType": "爆发",
    "missionKey": "Volatile",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Beacon Shield Ring (Venus)",
    "tier": "古纪",
    "tierNum": 1,
    "expired": false,
    "eta": "1h 12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4f61a69998d076c1c18f",
    "activation": "2025-02-23T17:40:01.402Z",
    "startString": "-17m 58s",
    "expiry": "2025-02-23T19:10:01.402Z",
    "active": true,
    "node": "薄暮星团 (海王星比邻星域)",
    "missionType": "間諜",
    "missionKey": "Spy",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Brom Cluster (Neptune)",
    "tier": "中纪",
    "tierNum": 3,
    "expired": false,
    "eta": "1h 12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4f61a69998d076c1c18e",
    "activation": "2025-02-23T17:40:01.401Z",
    "startString": "-17m 58s",
    "expiry": "2025-02-23T19:10:01.401Z",
    "active": true,
    "node": "水域星团 (土星比邻星域)",
    "missionType": "前哨战",
    "missionKey": "Skirmish",
    "enemy": "Grineer",
    "enemyKey": "Grineer",
    "nodeKey": "Vand Cluster (Saturn)",
    "tier": "前纪",
    "tierNum": 2,
    "expired": false,
    "eta": "1h 12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4f61a69998d076c1c190",
    "activation": "2025-02-23T17:40:01.403Z",
    "startString": "-17m 58s",
    "expiry": "2025-02-23T19:10:01.403Z",
    "active": true,
    "node": "外域星轴 (冥王星比邻星域)",
    "missionType": "奧菲斯",
    "missionKey": "Orphix",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Peregrine Axis (Pluto)",
    "tier": "后纪",
    "tierNum": 4,
    "expired": false,
    "eta": "1h 12m 1s",
    "isStorm": true,
    "isHard": false
  },
  {
    "id": "67bb4f61a69998d076c1c191",
    "activation": "2025-02-23T17:40:01.403Z",
    "startString": "-17m 58s",
    "expiry": "2025-02-23T19:10:01.403Z",
    "active": true,
    "node": "蒲芦 (面纱)",
    "missionType": "殲滅",
    "missionKey": "Extermination",
    "enemy": "Corpus",
    "enemyKey": "Corpus",
    "nodeKey": "Calabash (Veil)",
    "tier": "后纪",
    "tierNum": 4,
    "expired": false,
    "eta": "1h 12m 1s",
    "isStorm": true,
    "isHard": false
  }
]
"#
}
