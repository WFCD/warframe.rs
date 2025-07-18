use crate::fixture;

fixture! {
deep_archimedea,
r#"
{
  "id": "1743379200000DeepArchimedea",
  "activation": "2025-03-31T00:00:00.000Z",
  "expiry": "2025-04-07T00:00:00.000Z",
  "missions": [
    {
      "mission": "Extermination",
      "deviation": {
        "key": "GrowingIncursion",
        "name": "Fissure Cascade",
        "description": "Fissures rip into the mission, causing the Enemy Level to go up by 1 every 10s. Destroy them to stop the level from increasing further."
      },
      "riskVariables": [
        {
          "key": "AcceleratedEnemies",
          "name": "Bold Venture",
          "description": "Enemies deal -15% Damage and take +15% Damage but gain +15% Movement Speed, Attack Speed, and Fire Rate."
        },
        {
          "key": "EMPBlackHole",
          "name": "Alluring Arcocanids",
          "description": "As Rogue Arcocanids charge attacks, they pull Warframes towards them."
        }
      ]
    },
    {
      "mission": "Disruption",
      "deviation": {
        "key": "FragileNodes",
        "name": "Unified Purpose",
        "description": "Enemies can target and destroy Conduits."
      },
      "riskVariables": [
        {
          "key": "ExplosiveCrawlers",
          "name": "Explosive Potential",
          "description": "Rupturing Fragments replace Shuffling Fragments."
        },
        {
          "key": "Voidburst",
          "name": "Postmortal Surges",
          "description": "Slain enemies burts with Void energy."
        }
      ]
    },
    {
      "mission": "Assassination",
      "deviation": {
        "key": "InfiniteTide",
        "name": "Relentless Tide",
        "description": "The Fragmented Tide never stops attacking."
      },
      "riskVariables": [
        {
          "key": "PointBlank",
          "name": "Myopic Munitions",
          "description": "Enemies will only take damage if a player is within 15m of them."
        },
        {
          "key": "ShieldedFoes",
          "name": "Bolstered Belligerents",
          "description": "All enemies have Overguard equal to 50% of their max health."
        }
      ]
    }
  ],
  "personalModifiers": [
    {
      "key": "ContactDamage",
      "name": "Secondary Wounds",
      "description": "Gain 1 Puncture Status Effect every time you take damage."
    },
    {
      "key": "Gearless",
      "name": "Gear Embargo",
      "description": "Gear cannot be used."
    },
    {
      "key": "Armorless",
      "name": "Fractured Armor",
      "description": "Casting an ability reduces armor by 10% for 10s."
    },
    {
      "key": "AbilityLockout",
      "name": "Powerless",
      "description": "All Abilities are disabled until the squad kills 50 enemies."
    }
  ]
}
"#
---
r#"
{
  "id": "1743379200000DeepArchimedea",
  "activation": "2025-03-31T00:00:00.000Z",
  "expiry": "2025-04-07T00:00:00.000Z",
  "missions": [
    {
      "mission": "Extermination",
      "deviation": {
        "key": "GrowingIncursion",
        "name": "Fissure Cascade",
        "description": "Fissures rip into the mission, causing the Enemy Level to go up by 1 every 10s. Destroy them to stop the level from increasing further."
      },
      "riskVariables": [
        {
          "key": "AcceleratedEnemies",
          "name": "Bold Venture",
          "description": "Enemies deal -15% Damage and take +15% Damage but gain +15% Movement Speed, Attack Speed, and Fire Rate."
        },
        {
          "key": "EMPBlackHole",
          "name": "Alluring Arcocanids",
          "description": "As Rogue Arcocanids charge attacks, they pull Warframes towards them."
        }
      ]
    },
    {
      "mission": "Disruption",
      "deviation": {
        "key": "FragileNodes",
        "name": "Unified Purpose",
        "description": "Enemies can target and destroy Conduits."
      },
      "riskVariables": [
        {
          "key": "ExplosiveCrawlers",
          "name": "Explosive Potential",
          "description": "Rupturing Fragments replace Shuffling Fragments."
        },
        {
          "key": "Voidburst",
          "name": "Postmortal Surges",
          "description": "Slain enemies burts with Void energy."
        }
      ]
    },
    {
      "mission": "Assassination",
      "deviation": {
        "key": "InfiniteTide",
        "name": "Relentless Tide",
        "description": "The Fragmented Tide never stops attacking."
      },
      "riskVariables": [
        {
          "key": "PointBlank",
          "name": "Myopic Munitions",
          "description": "Enemies will only take damage if a player is within 15m of them."
        },
        {
          "key": "ShieldedFoes",
          "name": "Bolstered Belligerents",
          "description": "All enemies have Overguard equal to 50% of their max health."
        }
      ]
    }
  ],
  "personalModifiers": [
    {
      "key": "ContactDamage",
      "name": "Secondary Wounds",
      "description": "Gain 1 Puncture Status Effect every time you take damage."
    },
    {
      "key": "Gearless",
      "name": "Gear Embargo",
      "description": "Gear cannot be used."
    },
    {
      "key": "Armorless",
      "name": "Fractured Armor",
      "description": "Casting an ability reduces armor by 10% for 10s."
    },
    {
      "key": "AbilityLockout",
      "name": "Powerless",
      "description": "All Abilities are disabled until the squad kills 50 enemies."
    }
  ]
}
"#
}
