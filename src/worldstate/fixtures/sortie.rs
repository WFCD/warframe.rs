use crate::fixture;

fixture! {
    sortie,
r#"
{
  "id": "67bb508fa44542e407c1c18d",
  "activation": "2025-02-23T17:00:00.000Z",
  "startString": "-1h 14m 10s",
  "expiry": "2025-02-24T17:00:00.000Z",
  "active": true,
  "rewardPool": "Sortie Rewards",
  "variants": [
    {
      "missionType": "Sabotage",
      "missionTypeKey": "Sabotage",
      "modifier": "Eximus Stronghold",
      "modifierDescription": "Eximus units have a much higher spawn rate in this mission. Some of their auras stack.",
      "node": "Outer Terminus (Pluto)",
      "nodeKey": "Outer Terminus (Pluto)"
    },
    {
      "missionType": "Spy",
      "missionTypeKey": "Spy",
      "modifier": "Enhanced Enemy Shields",
      "modifierDescription": "Enemies have vastly Improved Shields. Magnetic Procs are advised.",
      "node": "Aphrodite (Venus)",
      "nodeKey": "Aphrodite (Venus)"
    },
    {
      "missionType": "Mobile Defense",
      "missionTypeKey": "Mobile Defense",
      "modifier": "Enemy Elemental Enhancement: Blast",
      "modifierDescription": "Enemies deal increased Blast damage and also have increased Immunity to said damage type.",
      "node": "Roche (Phobos)",
      "nodeKey": "Roche (Phobos)"
    }
  ],
  "missions": [],
  "boss": "Ambulas",
  "faction": "Corpus",
  "factionKey": "Corpus",
  "expired": false,
  "eta": "22h 45m 49s"
}
"#
---
r#"
{
  "id": "67bb508fa44542e407c1c18d",
  "activation": "2025-02-23T17:00:00.000Z",
  "startString": "-1h 14m 40s",
  "expiry": "2025-02-24T17:00:00.000Z",
  "active": true,
  "rewardPool": "Sortie Rewards",
  "variants": [
    {
      "missionType": "破壞",
      "missionTypeKey": "Sabotage",
      "modifier": "卓越者大本营",
      "modifierDescription": "卓越者单位在这个任务中的生成率更高，并且携带的光环数量更多。",
      "node": "Outer Terminus (冥王星)",
      "nodeKey": "Outer Terminus (Pluto)"
    },
    {
      "missionType": "間諜",
      "missionTypeKey": "Spy",
      "modifier": "敌人护盾强化",
      "modifierDescription": "敵人已有強化護盾，建議使用磁力元素。",
      "node": "Aphrodite (金星)",
      "nodeKey": "Aphrodite (Venus)"
    },
    {
      "missionType": "移動防禦",
      "missionTypeKey": "Mobile Defense",
      "modifier": "敌人元素强化(爆炸)",
      "modifierDescription": "敌人造成更高的爆炸伤害，并对爆炸伤害具有更高的抗性",
      "node": "Roche (火衛一)",
      "nodeKey": "Roche (Phobos)"
    }
  ],
  "missions": [],
  "boss": "Ambulas",
  "faction": "Corpus",
  "factionKey": "Corpus",
  "expired": false,
  "eta": "22h 45m 19s"
}
"#
}
