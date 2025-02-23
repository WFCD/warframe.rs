use crate::fixture;

fixture! {
    ranged_weapon,
r#"
{
  "accuracy": 23.529411,
  "attacks": [
    {
      "name": "Rocket Impact",
      "speed": 10,
      "crit_chance": 34,
      "crit_mult": 3,
      "status_chance": 18,
      "shot_type": "Projectile",
      "shot_speed": 70,
      "flight": 70,
      "damage": {
        "impact": 44
      }
    },
    {
      "name": "Rocket Explosion",
      "speed": 10,
      "crit_chance": 34,
      "crit_mult": 3,
      "status_chance": 18,
      "shot_type": "AoE",
      "falloff": {
        "start": 0,
        "end": 5,
        "reduction": 0.5
      },
      "damage": {
        "slash": 10.6,
        "puncture": 42.4
      }
    }
  ],
  "buildPrice": 15000,
  "buildQuantity": 1,
  "buildTime": 43200,
  "category": "Primary",
  "components": [
    {
      "uniqueName": "/Lotus/Types/Recipes/Weapons/WeaponParts/AcceltraPrimeBarrel",
      "name": "Barrel",
      "description": "A prime weapon-crafting component.",
      "primeSellingPrice": 100,
      "itemCount": 1,
      "imageName": "prime-barrel.png",
      "tradable": true,
      "drops": [
        {
          "chance": 0.02,
          "location": "Meso A5 Relic",
          "rarity": "Rare",
          "type": "Acceltra Prime Barrel",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionGaussPrimeCBronze"
        },
        {
          "chance": 0.02,
          "location": "Neo A12 Relic",
          "rarity": "Rare",
          "type": "Acceltra Prime Barrel",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionProteaPrimeCBronze"
        },
        {
          "chance": 0.02,
          "location": "Neo A13 Relic",
          "rarity": "Rare",
          "type": "Acceltra Prime Barrel",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionSevagothPrimeBBronze"
        },
        {
          "chance": 0.04,
          "location": "Meso A5 Relic (Exceptional)",
          "rarity": "Rare",
          "type": "Acceltra Prime Barrel",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionGaussPrimeCBronze"
        },
        {
          "chance": 0.04,
          "location": "Neo A12 Relic (Exceptional)",
          "rarity": "Rare",
          "type": "Acceltra Prime Barrel",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionProteaPrimeCBronze"
        },
        {
          "chance": 0.04,
          "location": "Neo A13 Relic (Exceptional)",
          "rarity": "Rare",
          "type": "Acceltra Prime Barrel",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionSevagothPrimeBBronze"
        },
        {
          "chance": 0.06,
          "location": "Meso A5 Relic (Flawless)",
          "rarity": "Rare",
          "type": "Acceltra Prime Barrel",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionGaussPrimeCBronze"
        },
        {
          "chance": 0.06,
          "location": "Neo A12 Relic (Flawless)",
          "rarity": "Rare",
          "type": "Acceltra Prime Barrel",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionProteaPrimeCBronze"
        },
        {
          "chance": 0.06,
          "location": "Neo A13 Relic (Flawless)",
          "rarity": "Rare",
          "type": "Acceltra Prime Barrel",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionSevagothPrimeBBronze"
        },
        {
          "chance": 0.1,
          "location": "Meso A5 Relic (Radiant)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Barrel",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionGaussPrimeCBronze"
        },
        {
          "chance": 0.1,
          "location": "Neo A12 Relic (Radiant)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Barrel",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionProteaPrimeCBronze"
        },
        {
          "chance": 0.1,
          "location": "Neo A13 Relic (Radiant)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Barrel",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionSevagothPrimeBBronze"
        }
      ],
      "masterable": false,
      "ducats": 100
    },
    {
      "uniqueName": "/Lotus/Types/Recipes/Weapons/AcceltraPrimeBlueprint",
      "name": "Blueprint",
      "description": "Engage your enemies with deadly speed. This weapon reloads even faster when its wielder sprints, faster still with Gauss.",
      "itemCount": 1,
      "primeSellingPrice": 15,
      "imageName": "blueprint.png",
      "tradable": true,
      "masterable": false,
      "ducats": 15,
      "drops": [
        {
          "chance": 0.1667,
          "location": "Axi M6 Relic (Radiant)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionLavosPrimeDBronze"
        },
        {
          "chance": 0.1667,
          "location": "Lith C11 Relic (Radiant)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T1VoidProjectionGaussPrimeABronze"
        },
        {
          "chance": 0.1667,
          "location": "Meso N17 Relic (Radiant)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionSevagothPrimeABronze"
        },
        {
          "chance": 0.1667,
          "location": "Meso V10 Relic (Radiant)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionXakuPrimeDBronze"
        },
        {
          "chance": 0.1667,
          "location": "Meso W5 Relic (Radiant)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionProteaPrimeDBronze"
        },
        {
          "chance": 0.1667,
          "location": "Neo Z11 Relic (Radiant)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionSevagothPrimeDBronze"
        },
        {
          "chance": 0.2,
          "location": "Axi M6 Relic (Flawless)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionLavosPrimeDBronze"
        },
        {
          "chance": 0.2,
          "location": "Lith C11 Relic (Flawless)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T1VoidProjectionGaussPrimeABronze"
        },
        {
          "chance": 0.2,
          "location": "Meso N17 Relic (Flawless)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionSevagothPrimeABronze"
        },
        {
          "chance": 0.2,
          "location": "Meso V10 Relic (Flawless)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionXakuPrimeDBronze"
        },
        {
          "chance": 0.2,
          "location": "Meso W5 Relic (Flawless)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionProteaPrimeDBronze"
        },
        {
          "chance": 0.2,
          "location": "Neo Z11 Relic (Flawless)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionSevagothPrimeDBronze"
        },
        {
          "chance": 0.2333,
          "location": "Axi M6 Relic (Exceptional)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionLavosPrimeDBronze"
        },
        {
          "chance": 0.2333,
          "location": "Lith C11 Relic (Exceptional)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T1VoidProjectionGaussPrimeABronze"
        },
        {
          "chance": 0.2333,
          "location": "Meso N17 Relic (Exceptional)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionSevagothPrimeABronze"
        },
        {
          "chance": 0.2333,
          "location": "Meso V10 Relic (Exceptional)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionXakuPrimeDBronze"
        },
        {
          "chance": 0.2333,
          "location": "Meso W5 Relic (Exceptional)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionProteaPrimeDBronze"
        },
        {
          "chance": 0.2333,
          "location": "Neo Z11 Relic (Exceptional)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionSevagothPrimeDBronze"
        },
        {
          "chance": 0.2533,
          "location": "Axi M6 Relic",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionLavosPrimeDBronze"
        },
        {
          "chance": 0.2533,
          "location": "Lith C11 Relic",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T1VoidProjectionGaussPrimeABronze"
        },
        {
          "chance": 0.2533,
          "location": "Meso N17 Relic",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionSevagothPrimeABronze"
        },
        {
          "chance": 0.2533,
          "location": "Meso V10 Relic",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionXakuPrimeDBronze"
        },
        {
          "chance": 0.2533,
          "location": "Meso W5 Relic",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionProteaPrimeDBronze"
        },
        {
          "chance": 0.2533,
          "location": "Neo Z11 Relic",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Blueprint",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionSevagothPrimeDBronze"
        }
      ]
    },
    {
      "uniqueName": "/Lotus/Types/Items/MiscItems/OrokinCell",
      "name": "Orokin Cell",
      "description": "Ancient energy cell from the Orokin era.Location: Ceres, Saturn, and Deimos",
      "itemCount": 10,
      "imageName": "orokin-cell-0d237af036.png",
      "tradable": false,
      "drops": [],
      "masterable": false,
      "type": "Resource"
    },
    {
      "uniqueName": "/Lotus/Types/Recipes/Weapons/WeaponParts/AcceltraPrimeReceiver",
      "name": "Receiver",
      "description": "A prime weapon-crafting component.",
      "primeSellingPrice": 45,
      "itemCount": 1,
      "imageName": "prime-receiver.png",
      "tradable": true,
      "drops": [
        {
          "chance": 0.11,
          "location": "Axi O6 Relic",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionSevagothPrimeABronze"
        },
        {
          "chance": 0.11,
          "location": "Axi Z2 Relic",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionLavosPrimeCBronze"
        },
        {
          "chance": 0.11,
          "location": "Neo F3 Relic",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionGaussPrimeABronze"
        },
        {
          "chance": 0.11,
          "location": "Neo L4 Relic",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionXakuPrimeABronze"
        },
        {
          "chance": 0.11,
          "location": "Neo O2 Relic",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionProteaPrimeABronze"
        },
        {
          "chance": 0.13,
          "location": "Axi O6 Relic (Exceptional)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionSevagothPrimeABronze"
        },
        {
          "chance": 0.13,
          "location": "Axi Z2 Relic (Exceptional)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionLavosPrimeCBronze"
        },
        {
          "chance": 0.13,
          "location": "Neo F3 Relic (Exceptional)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionGaussPrimeABronze"
        },
        {
          "chance": 0.13,
          "location": "Neo L4 Relic (Exceptional)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionXakuPrimeABronze"
        },
        {
          "chance": 0.13,
          "location": "Neo O2 Relic (Exceptional)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionProteaPrimeABronze"
        },
        {
          "chance": 0.17,
          "location": "Axi O6 Relic (Flawless)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionSevagothPrimeABronze"
        },
        {
          "chance": 0.17,
          "location": "Axi Z2 Relic (Flawless)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionLavosPrimeCBronze"
        },
        {
          "chance": 0.17,
          "location": "Neo F3 Relic (Flawless)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionGaussPrimeABronze"
        },
        {
          "chance": 0.17,
          "location": "Neo L4 Relic (Flawless)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionXakuPrimeABronze"
        },
        {
          "chance": 0.17,
          "location": "Neo O2 Relic (Flawless)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionProteaPrimeABronze"
        },
        {
          "chance": 0.2,
          "location": "Axi O6 Relic (Radiant)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionSevagothPrimeABronze"
        },
        {
          "chance": 0.2,
          "location": "Axi Z2 Relic (Radiant)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionLavosPrimeCBronze"
        },
        {
          "chance": 0.2,
          "location": "Neo F3 Relic (Radiant)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionGaussPrimeABronze"
        },
        {
          "chance": 0.2,
          "location": "Neo L4 Relic (Radiant)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionXakuPrimeABronze"
        },
        {
          "chance": 0.2,
          "location": "Neo O2 Relic (Radiant)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Receiver",
          "uniqueName": "/Lotus/Types/Game/Projections/T3VoidProjectionProteaPrimeABronze"
        }
      ],
      "masterable": false,
      "ducats": 45
    },
    {
      "uniqueName": "/Lotus/Types/Recipes/Weapons/WeaponParts/AcceltraPrimeStock",
      "name": "Stock",
      "description": "A prime weapon-crafting component.",
      "primeSellingPrice": 100,
      "itemCount": 1,
      "imageName": "prime-stock.png",
      "tradable": true,
      "drops": [
        {
          "chance": 0.02,
          "location": "Axi A18 Relic",
          "rarity": "Rare",
          "type": "Acceltra Prime Stock",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionGaussPrimeDBronze"
        },
        {
          "chance": 0.02,
          "location": "Axi A19 Relic",
          "rarity": "Rare",
          "type": "Acceltra Prime Stock",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionXakuPrimeDBronze"
        },
        {
          "chance": 0.02,
          "location": "Meso A8 Relic",
          "rarity": "Rare",
          "type": "Acceltra Prime Stock",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionLavosPrimeABronze"
        },
        {
          "chance": 0.04,
          "location": "Axi A18 Relic (Exceptional)",
          "rarity": "Rare",
          "type": "Acceltra Prime Stock",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionGaussPrimeDBronze"
        },
        {
          "chance": 0.04,
          "location": "Axi A19 Relic (Exceptional)",
          "rarity": "Rare",
          "type": "Acceltra Prime Stock",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionXakuPrimeDBronze"
        },
        {
          "chance": 0.04,
          "location": "Meso A8 Relic (Exceptional)",
          "rarity": "Rare",
          "type": "Acceltra Prime Stock",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionLavosPrimeABronze"
        },
        {
          "chance": 0.06,
          "location": "Axi A18 Relic (Flawless)",
          "rarity": "Rare",
          "type": "Acceltra Prime Stock",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionGaussPrimeDBronze"
        },
        {
          "chance": 0.06,
          "location": "Axi A19 Relic (Flawless)",
          "rarity": "Rare",
          "type": "Acceltra Prime Stock",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionXakuPrimeDBronze"
        },
        {
          "chance": 0.06,
          "location": "Meso A8 Relic (Flawless)",
          "rarity": "Rare",
          "type": "Acceltra Prime Stock",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionLavosPrimeABronze"
        },
        {
          "chance": 0.1,
          "location": "Axi A18 Relic (Radiant)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Stock",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionGaussPrimeDBronze"
        },
        {
          "chance": 0.1,
          "location": "Axi A19 Relic (Radiant)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Stock",
          "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionXakuPrimeDBronze"
        },
        {
          "chance": 0.1,
          "location": "Meso A8 Relic (Radiant)",
          "rarity": "Uncommon",
          "type": "Acceltra Prime Stock",
          "uniqueName": "/Lotus/Types/Game/Projections/T2VoidProjectionLavosPrimeABronze"
        }
      ],
      "masterable": false,
      "ducats": 100
    }
  ],
  "consumeOnBuild": true,
  "criticalChance": 0.34,
  "criticalMultiplier": 3,
  "damage": {
    "total": 97,
    "impact": 44,
    "puncture": 10.599999,
    "slash": 42.400002,
    "heat": 0,
    "cold": 0,
    "electricity": 0,
    "toxin": 0,
    "blast": 0,
    "radiation": 0,
    "gas": 0,
    "magnetic": 0,
    "viral": 0,
    "corrosive": 0,
    "void": 0,
    "tau": 0,
    "cinematic": 0,
    "shieldDrain": 0,
    "healthDrain": 0,
    "energyDrain": 0,
    "true": 0
  },
  "damagePerShot": [44, 42.400002, 10.599999, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  "description": "Engage your enemies with deadly speed. This weapon reloads even faster when its wielder sprints, faster still with Gauss.",
  "disposition": 1,
  "fireRate": 10.000001,
  "imageName": "acceltra-prime-5628f3e466.png",
  "introduced": {
    "name": "Hotfix 35.0.9",
    "url": "https://warframe.fandom.com/wiki/Update_35%23Hotfix_35.0.9",
    "aliases": [
      "35.0.9"
    ],
    "parent": "35.0",
    "date": "2024-01-17"
  },
  "isPrime": true,
  "magazineSize": 48,
  "masterable": true,
  "masteryReq": 14,
  "multishot": 1,
  "name": "Acceltra Prime",
  "noise": "Alarming",
  "omegaAttenuation": 0.55000001,
  "polarities": [
    "naramon",
    "madurai"
  ],
  "procChance": 0.18000001,
  "productCategory": "LongGuns",
  "releaseDate": "2024-01-17",
  "reloadTime": 1.6,
  "skipBuildTimePrice": 50,
  "slot": 1,
  "tags": [
    "Tenno",
    "Prime"
  ],
  "totalDamage": 97,
  "tradable": false,
  "trigger": "Auto",
  "type": "Rifle",
  "uniqueName": "/Lotus/Weapons/Tenno/LongGuns/PrimeAcceltra/PrimeAcceltraWeapon",
  "vaulted": false,
  "wikiaThumbnail": "https://static.wikia.nocookie.net/warframe/images/e/ec/AcceltraPrime.png/revision/latest?cb=20240117172955",
  "wikiaUrl": "https://warframe.fandom.com/wiki/Acceltra_Prime"
}
"#
}

fixture! {
    melee_weapon,
r#"
{
  "attacks": [
    {
      "name": "Normal Attack",
      "speed": 0.667,
      "crit_chance": 15,
      "crit_mult": 2.1,
      "status_chance": 30,
      "damage": {
        "impact": 57,
        "slash": 61,
        "puncture": 55,
        "viral": 89
      },
      "slide": "524",
      "slam": {
        "damage": "524.00",
        "radial": {
          "damage": "0.00",
          "radius": 5
        }
      }
    },
    {
      "name": "Throw",
      "speed": 0.667,
      "crit_chance": 17,
      "crit_mult": 2.3,
      "status_chance": 33,
      "shot_type": "Thrown",
      "shot_speed": 30,
      "flight": 30,
      "damage": {
        "impact": 49,
        "slash": 78,
        "puncture": 43,
        "viral": 118
      }
    },
    {
      "name": "Throw Bounce Explosion",
      "speed": 0.667,
      "crit_chance": 17,
      "crit_mult": 2.3,
      "status_chance": 33,
      "shot_type": "AoE",
      "falloff": {
        "start": 0,
        "end": 4.9,
        "reduction": 0.5
      },
      "damage": {
        "viral": 393
      }
    },
    {
      "name": "Throw Recall Explosion",
      "speed": 0.667,
      "crit_chance": 17,
      "crit_mult": 2.3,
      "status_chance": 33,
      "shot_type": "AoE",
      "falloff": {
        "start": 0,
        "end": 4.9,
        "reduction": 0
      },
      "damage": {
        "viral": 786
      }
    },
    {
      "name": "Charged Throw",
      "speed": 0.833,
      "crit_chance": 21,
      "crit_mult": 2.5,
      "status_chance": 35,
      "charge_time": 1.2,
      "shot_type": "Thrown",
      "shot_speed": 30,
      "flight": 30,
      "damage": {
        "impact": 127,
        "slash": 135,
        "puncture": 121,
        "viral": 193
      }
    },
    {
      "name": "Charged Throw Bounce Explosion",
      "speed": 0.833,
      "crit_chance": 21,
      "crit_mult": 2.5,
      "status_chance": 35,
      "charge_time": 1.2,
      "shot_type": "AoE",
      "falloff": {
        "start": 0,
        "end": 4.9,
        "reduction": 0.5
      },
      "damage": {
        "viral": 786
      }
    },
    {
      "name": "Charged Throw Recall Explosion",
      "speed": 0.833,
      "crit_chance": 21,
      "crit_mult": 2.5,
      "status_chance": 35,
      "charge_time": 1.2,
      "shot_type": "AoE",
      "falloff": {
        "start": 0,
        "end": 4.9,
        "reduction": 0
      },
      "damage": {
        "viral": 1572
      }
    }
  ],
  "blockingAngle": 55,
  "buildPrice": 30000,
  "buildQuantity": 1,
  "buildTime": 86400,
  "category": "Melee",
  "comboDuration": 5,
  "components": [
    {
      "uniqueName": "/Lotus/Types/Recipes/Weapons/WeaponParts/InfBoomerangBlade",
      "name": "Blade",
      "description": "A weapon-crafting component.",
      "itemCount": 2,
      "imageName": "blade.png",
      "tradable": false,
      "drops": [
        {
          "chance": 0.3333,
          "location": "Zealoid Prelate",
          "rarity": "Common",
          "type": "Pathocyst Blade"
        }
      ],
      "masterable": false
    },
    {
      "uniqueName": "/Lotus/Types/Recipes/Weapons/InfBoomerangBlueprint",
      "name": "Blueprint",
      "description": "Each strike of this infested glaive infects its target with viral pathogens. Occasionally, it discharges the spores of rabid, enemy-seeking maggots, either in-flight or on contact with the enemy.",
      "itemCount": 1,
      "imageName": "blueprint.png",
      "tradable": false,
      "masterable": false,
      "drops": [
        {
          "chance": 0.3333,
          "location": "Zealoid Prelate",
          "rarity": "Common",
          "type": "Pathocyst Blueprint"
        }
      ]
    },
    {
      "uniqueName": "/Lotus/Types/Items/MiscItems/Neurode",
      "name": "Neurodes",
      "description": "Biotech sensor organ harvested from Infested entities.Location: Earth, Lua, Eris, and Deimos",
      "itemCount": 5,
      "imageName": "neurodes-c027fd4a28.png",
      "tradable": false,
      "drops": [],
      "masterable": false
    },
    {
      "uniqueName": "/Lotus/Types/Recipes/Weapons/WeaponParts/InfBoomerangDisc",
      "name": "Subcortex",
      "description": "A weapon-crafting component.",
      "itemCount": 1,
      "imageName": "subcortex.png",
      "tradable": false,
      "drops": [
        {
          "chance": 0.3333,
          "location": "Zealoid Prelate",
          "rarity": "Common",
          "type": "Pathocyst Subcortex"
        }
      ],
      "masterable": false
    }
  ],
  "consumeOnBuild": true,
  "criticalChance": 0.15000001,
  "criticalMultiplier": 2.0999999,
  "damage": {
    "total": 262,
    "impact": 57,
    "puncture": 61,
    "slash": 55,
    "heat": 0,
    "cold": 0,
    "electricity": 0,
    "toxin": 0,
    "blast": 0,
    "radiation": 0,
    "gas": 0,
    "magnetic": 0,
    "viral": 89,
    "corrosive": 0,
    "void": 0,
    "tau": 0,
    "cinematic": 0,
    "shieldDrain": 0,
    "healthDrain": 0,
    "energyDrain": 0,
    "true": 0
  },
  "damagePerShot": [57, 55, 61, 0, 0, 0, 0, 0, 0, 0, 0, 89, 0, 0, 0, 0, 0, 0, 0, 0],
  "description": "Each strike of this infested glaive infects its target with viral pathogens. Occasionally, it discharges the spores of rabid, enemy-seeking maggots, either in-flight or on contact with the enemy.",
  "disposition": 4,
  "fireRate": 0.66666669,
  "followThrough": 0.69999999,
  "heavyAttackDamage": 262,
  "heavySlamAttack": 1048,
  "heavySlamRadialDamage": 786,
  "heavySlamRadius": 6,
  "imageName": "pathocyst-7c3a6d4c28.png",
  "introduced": {
    "name": "Hotfix 25.7.7",
    "url": "https://warframe.fandom.com/wiki/Update_25%23Hotfix_25.7.7",
    "aliases": [
      "25.7.7"
    ],
    "parent": "25.7",
    "date": "2019-09-26"
  },
  "isPrime": false,
  "masterable": true,
  "masteryReq": 9,
  "name": "Pathocyst",
  "omegaAttenuation": 1.3,
  "patchlogs": [
    {
      "name": "Update 30.5: Sisters of Parvos",
      "date": "2021-07-06T15:02:10Z",
      "url": "https://forums.warframe.com/topic/1269749-update-305-sisters-of-parvos/",
      "additions": "",
      "changes": "Pathocyst",
      "fixes": ""
    },
    {
      "name": "Orphix Venom: Hotfix 29.6.5",
      "date": "2021-01-08T21:15:03Z",
      "url": "https://forums.warframe.com/topic/1244435-orphix-venom-hotfix-2965/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Pathocyst Infested pod sound looping when pods are created but do not spawn a maggot."
    },
    {
      "name": "Heart of Deimos: TennoGen: 29.3.2",
      "date": "2020-11-05T21:04:26Z",
      "url": "https://forums.warframe.com/topic/1234372-heart-of-deimos-tennogen-2932/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed a heavy spot-load occurring when equipping the Pathocyst in the Arsenal."
    },
    {
      "name": "Warframe Revised: Hotfix 27.2.2",
      "date": "2020-03-06T20:01:27Z",
      "url": "https://forums.warframe.com/topic/1173118-warframe-revised-hotfix-2722/",
      "additions": "",
      "changes": "Pathocyst: 50%",
      "fixes": ""
    },
    {
      "name": "Atlas Prime: Update 25.8.0 + 25.8.0.1",
      "date": "2019-10-01T18:00:28Z",
      "url": "https://forums.warframe.com/topic/1131643-atlas-prime-update-2580-25801/",
      "additions": "Added new “fast whooshing” sounds to the Pathocyst.",
      "changes": "",
      "fixes": ""
    }
  ],
  "polarities": [
    "madurai",
    "vazarin"
  ],
  "procChance": 0.30000001,
  "productCategory": "Melee",
  "range": 1.3,
  "releaseDate": "2019-09-26",
  "skipBuildTimePrice": 25,
  "slamAttack": 786,
  "slamRadialDamage": 524,
  "slamRadius": 5,
  "slideAttack": 524,
  "slot": 5,
  "stancePolarity": "naramon",
  "tags": [
    "Infested"
  ],
  "totalDamage": 262,
  "tradable": false,
  "type": "Melee",
  "uniqueName": "/Lotus/Weapons/Infested/Melee/InfBoomerang/InfBoomerangWeapon",
  "wikiaThumbnail": "https://static.wikia.nocookie.net/warframe/images/e/ec/Pathocyst.png/revision/latest?cb=20220612022533",
  "wikiaUrl": "https://warframe.fandom.com/wiki/Pathocyst",
  "windUp": 1.2
}
"#
}
