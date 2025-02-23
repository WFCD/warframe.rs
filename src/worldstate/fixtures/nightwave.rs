use crate::fixture;

fixture! {
    nightwave,
r#"
{
  "id": "nightwave1756684800000",
  "activation": "2025-02-06T19:15:00.000Z",
  "startString": "-16d 22h 56m 30s",
  "expiry": "2025-09-01T00:00:00.000Z",
  "active": true,
  "season": 14,
  "tag": "Radio Legion Intermission12 Syndicate",
  "phase": 0,
  "params": {

  },
  "possibleChallenges": [],
  "activeChallenges": [
    {
      "id": "1740355200000seasondailykillenemieswithfire",
      "activation": "2025-02-21T00:00:00.000Z",
      "startString": "-2d 18h 11m 30s",
      "expiry": "2025-02-24T00:00:00.000Z",
      "active": true,
      "isDaily": true,
      "isElite": false,
      "desc": "Kill 150 Enemies with Heat Damage",
      "title": "Arsonist",
      "reputation": 1000,
      "isPermanent": false
    },
    {
      "id": "1740441600000seasondailykillenemieswithabilities",
      "activation": "2025-02-22T00:00:00.000Z",
      "startString": "-1d 18h 11m 30s",
      "expiry": "2025-02-25T00:00:00.000Z",
      "active": true,
      "isDaily": true,
      "isElite": false,
      "desc": "Kill 150 Enemies with Abilities",
      "title": "Power Trip",
      "reputation": 1000,
      "isPermanent": false
    },
    {
      "id": "1740528000000seasondailybulletjump",
      "activation": "2025-02-23T00:00:00.000Z",
      "startString": "-18h 11m 30s",
      "expiry": "2025-02-26T00:00:00.000Z",
      "active": true,
      "isDaily": true,
      "isElite": false,
      "desc": "Bullet Jump 150 times",
      "title": "Trampoline",
      "reputation": 1000,
      "isPermanent": false
    },
    {
      "id": "1740355200000seasonweeklypermanentcompletemissions3",
      "activation": "2025-02-17T00:00:00.000Z",
      "startString": "-6d 18h 11m 30s",
      "expiry": "2025-02-24T00:00:00.000Z",
      "active": true,
      "isDaily": false,
      "isElite": false,
      "desc": "Complete any 15 missions",
      "title": "Mission Complete III",
      "reputation": 4500,
      "isPermanent": false
    },
    {
      "id": "1740355200000seasonweeklypermanentkilleximus3",
      "activation": "2025-02-17T00:00:00.000Z",
      "startString": "-6d 18h 11m 30s",
      "expiry": "2025-02-24T00:00:00.000Z",
      "active": true,
      "isDaily": false,
      "isElite": false,
      "desc": "Kill 30 Eximus",
      "title": "Eximus Eliminator III",
      "reputation": 4500,
      "isPermanent": false
    },
    {
      "id": "1740355200000seasonweeklypermanentkillenemies3",
      "activation": "2025-02-17T00:00:00.000Z",
      "startString": "-6d 18h 11m 30s",
      "expiry": "2025-02-24T00:00:00.000Z",
      "active": true,
      "isDaily": false,
      "isElite": false,
      "desc": "Kill 500 Enemies",
      "title": "Not a Warning Shot III",
      "reputation": 4500,
      "isPermanent": false
    },
    {
      "id": "1740355200000seasonweeklyfeedhelminth",
      "activation": "2025-02-17T00:00:00.000Z",
      "startString": "-6d 18h 11m 30s",
      "expiry": "2025-02-24T00:00:00.000Z",
      "active": true,
      "isDaily": false,
      "isElite": false,
      "desc": "Feed the Helminth any resource",
      "title": "Feed The Beast",
      "reputation": 4500,
      "isPermanent": false
    },
    {
      "id": "1740355200000seasonweeklyvenusbounties",
      "activation": "2025-02-17T00:00:00.000Z",
      "startString": "-6d 18h 11m 30s",
      "expiry": "2025-02-24T00:00:00.000Z",
      "active": true,
      "isDaily": false,
      "isElite": false,
      "desc": "Complete 3 different bounties in the Orb Vallis",
      "title": "Venus Bounty Hunter",
      "reputation": 4500,
      "isPermanent": false
    },
    {
      "id": "1740355200000seasonweeklyhardluapuzzles",
      "activation": "2025-02-17T00:00:00.000Z",
      "startString": "-6d 18h 11m 30s",
      "expiry": "2025-02-24T00:00:00.000Z",
      "active": true,
      "isDaily": false,
      "isElite": true,
      "desc": "Complete 4 Halls of Ascension on Lua",
      "title": "Ascendant",
      "reputation": 7000,
      "isPermanent": false
    },
    {
      "id": "1740355200000seasonweeklyhardkillthumper",
      "activation": "2025-02-17T00:00:00.000Z",
      "startString": "-6d 18h 11m 30s",
      "expiry": "2025-02-24T00:00:00.000Z",
      "active": true,
      "isDaily": false,
      "isElite": true,
      "desc": "Kill a Tusk Thumper Doma in the Plains of Eidolon",
      "title": "Walk Without Rhythm",
      "reputation": 7000,
      "isPermanent": false
    }
  ],
  "rewardTypes": [
    "credits"
  ]
}
"#
---
r#"
{
  "id": "nightwave1756684800000",
  "activation": "2025-02-06T19:15:00.000Z",
  "startString": "-16d 22h 57m 30s",
  "expiry": "2025-09-01T00:00:00.000Z",
  "active": true,
  "season": 14,
  "tag": "Radio Legion Intermission12 Syndicate",
  "phase": 0,
  "params": {

  },
  "possibleChallenges": [],
  "activeChallenges": [
    {
      "id": "1740355200000seasondailykillenemieswithfire",
      "activation": "2025-02-21T00:00:00.000Z",
      "startString": "-2d 18h 12m 30s",
      "expiry": "2025-02-24T00:00:00.000Z",
      "active": true,
      "isDaily": true,
      "isElite": false,
      "desc": "使用火焰伤害击杀150名敌人",
      "title": "纵火犯",
      "reputation": 1000,
      "isPermanent": false
    },
    {
      "id": "1740441600000seasondailykillenemieswithabilities",
      "activation": "2025-02-22T00:00:00.000Z",
      "startString": "-1d 18h 12m 30s",
      "expiry": "2025-02-25T00:00:00.000Z",
      "active": true,
      "isDaily": true,
      "isElite": false,
      "desc": "使用技能擊殺150名敵人",
      "title": "力量展现",
      "reputation": 1000,
      "isPermanent": false
    },
    {
      "id": "1740528000000seasondailybulletjump",
      "activation": "2025-02-23T00:00:00.000Z",
      "startString": "-18h 12m 30s",
      "expiry": "2025-02-26T00:00:00.000Z",
      "active": true,
      "isDaily": true,
      "isElite": false,
      "desc": "旋身飞跃150次",
      "title": "蹦弹",
      "reputation": 1000,
      "isPermanent": false
    },
    {
      "id": "1740355200000seasonweeklypermanentcompletemissions3",
      "activation": "2025-02-17T00:00:00.000Z",
      "startString": "-6d 18h 12m 30s",
      "expiry": "2025-02-24T00:00:00.000Z",
      "active": true,
      "isDaily": false,
      "isElite": false,
      "desc": "完成任意 15 项任务",
      "title": "任务完成 III",
      "reputation": 4500,
      "isPermanent": false
    },
    {
      "id": "1740355200000seasonweeklypermanentkilleximus3",
      "activation": "2025-02-17T00:00:00.000Z",
      "startString": "-6d 18h 12m 30s",
      "expiry": "2025-02-24T00:00:00.000Z",
      "active": true,
      "isDaily": false,
      "isElite": false,
      "desc": "击杀30名卓越者",
      "title": "卓越歼灭者 III",
      "reputation": 4500,
      "isPermanent": false
    },
    {
      "id": "1740355200000seasonweeklypermanentkillenemies3",
      "activation": "2025-02-17T00:00:00.000Z",
      "startString": "-6d 18h 12m 30s",
      "expiry": "2025-02-24T00:00:00.000Z",
      "active": true,
      "isDaily": false,
      "isElite": false,
      "desc": "击杀500名敌人",
      "title": "这不是警告射击 III",
      "reputation": 4500,
      "isPermanent": false
    },
    {
      "id": "1740355200000seasonweeklyfeedhelminth",
      "activation": "2025-02-17T00:00:00.000Z",
      "startString": "-6d 18h 12m 30s",
      "expiry": "2025-02-24T00:00:00.000Z",
      "active": true,
      "isDaily": false,
      "isElite": false,
      "desc": "向 Helminth 饲喂任意资源",
      "title": "喂饱野兽",
      "reputation": 4500,
      "isPermanent": false
    },
    {
      "id": "1740355200000seasonweeklyvenusbounties",
      "activation": "2025-02-17T00:00:00.000Z",
      "startString": "-6d 18h 12m 30s",
      "expiry": "2025-02-24T00:00:00.000Z",
      "active": true,
      "isDaily": false,
      "isElite": false,
      "desc": "在奥布山谷完成 3 个不同的赏金任务",
      "title": "金星赏金猎人",
      "reputation": 4500,
      "isPermanent": false
    },
    {
      "id": "1740355200000seasonweeklyhardluapuzzles",
      "activation": "2025-02-17T00:00:00.000Z",
      "startString": "-6d 18h 12m 30s",
      "expiry": "2025-02-24T00:00:00.000Z",
      "active": true,
      "isDaily": false,
      "isElite": true,
      "desc": "完成月球上的4间晋升大厅",
      "title": "上行",
      "reputation": 7000,
      "isPermanent": false
    },
    {
      "id": "1740355200000seasonweeklyhardkillthumper",
      "activation": "2025-02-17T00:00:00.000Z",
      "startString": "-6d 18h 12m 30s",
      "expiry": "2025-02-24T00:00:00.000Z",
      "active": true,
      "isDaily": false,
      "isElite": true,
      "desc": "在夜灵平野上击杀一个巨牙重击者朵玛",
      "title": "无律之行",
      "reputation": 7000,
      "isPermanent": false
    }
  ],
  "rewardTypes": [
    "credits"
  ]
}
"#
}
