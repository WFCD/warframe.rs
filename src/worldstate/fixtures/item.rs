use crate::fixture;

fixture! {
    item_sigil,
r#"
{
  "category": "Sigils",
  "description": "A sigil representing prestige gained with the Conclave.",
  "drops": [
    {
      "chance": 1,
      "location": "Conclave, Hurricane",
      "rarity": "Common",
      "type": "Accord Sigil"
    }
  ],
  "imageName": "accord-sigil-a11257c0bb.png",
  "masterable": false,
  "name": "Accord Sigil",
  "tradable": false,
  "type": "Sigil",
  "uniqueName": "/Lotus/Upgrades/Skins/Sigils/SyndicateSigilConclaveN"
}
"#
}

#[rstest::fixture]
pub fn nanospores_de() -> &'static str {
    r#"
{
  "category": "Misc",
  "description": "Faseriger Technocyte-Tumor. Befallenes Gewebe ist mit Vorsicht zu behandeln.Quelle: Saturn, Neptun, Eris und Deimos.",
  "drops": [
    {
      "chance": 0.1265,
      "location": "Neptune/Cephalon Capture (Conclave), Rotation B",
      "rarity": "Uncommon",
      "type": "500X Nano Spores"
    },
    {
      "chance": 0.1265,
      "location": "Saturn/Annihilation (Conclave), Rotation B",
      "rarity": "Uncommon",
      "type": "500X Nano Spores"
    },
    {
      "chance": 0.1265,
      "location": "Saturn/Cephalon Capture (Conclave), Rotation B",
      "rarity": "Uncommon",
      "type": "500X Nano Spores"
    },
    {
      "chance": 0.1265,
      "location": "Saturn/Lunaro (Conclave), Rotation B",
      "rarity": "Uncommon",
      "type": "500X Nano Spores"
    },
    {
      "chance": 0.1265,
      "location": "Saturn/Team Annihilation (Conclave), Rotation B",
      "rarity": "Uncommon",
      "type": "500X Nano Spores"
    },
    {
      "chance": 0.1265,
      "location": "Saturn/Variant Annihilation (Conclave), Rotation B",
      "rarity": "Uncommon",
      "type": "500X Nano Spores"
    },
    {
      "chance": 0.1265,
      "location": "Saturn/Variant Cephalon Capture (Conclave), Rotation B",
      "rarity": "Uncommon",
      "type": "500X Nano Spores"
    },
    {
      "chance": 0.1265,
      "location": "Saturn/Variant Team Annihilation (Conclave), Rotation B",
      "rarity": "Uncommon",
      "type": "500X Nano Spores"
    },
    {
      "chance": 0.1324,
      "location": "Venus/Orb Vallis (Level 10 - 30 Orb Vallis Bounty), Rotation C",
      "rarity": "Uncommon",
      "type": "200X Nano Spores"
    },
    {
      "chance": 0.1429,
      "location": "Deimos/Formido (Caches), Rotation A",
      "rarity": "Uncommon",
      "type": "1000X Nano Spores"
    },
    {
      "chance": 0.15,
      "location": "Venus/Orb Vallis (Level 10 - 30 Orb Vallis Bounty), Rotation C",
      "rarity": "Uncommon",
      "type": "200X Nano Spores"
    },
    {
      "chance": 0.1518,
      "location": "Hallowed Flame Mission Caches, Rotation A",
      "rarity": "Uncommon",
      "type": "500X Nano Spores"
    },
    {
      "chance": 0.1667,
      "location": "Eris/Candiru (Caches), Rotation A",
      "rarity": "Uncommon",
      "type": "1000X Nano Spores"
    },
    {
      "chance": 0.1667,
      "location": "Eris/Lepis (Caches), Rotation A",
      "rarity": "Uncommon",
      "type": "1000X Nano Spores"
    },
    {
      "chance": 0.1667,
      "location": "Eris/Naeglar (Caches), Rotation A",
      "rarity": "Uncommon",
      "type": "1000X Nano Spores"
    },
    {
      "chance": 0.1667,
      "location": "Eris/Psoro (Caches), Rotation A",
      "rarity": "Uncommon",
      "type": "1000X Nano Spores"
    },
    {
      "chance": 0.1667,
      "location": "Eris/Viver (Caches), Rotation A",
      "rarity": "Uncommon",
      "type": "1000X Nano Spores"
    },
    {
      "chance": 0.25,
      "location": "Venus/Orb Vallis (Level 10 - 30 Orb Vallis Bounty), Rotation C",
      "rarity": "Uncommon",
      "type": "200X Nano Spores"
    }
  ],
  "imageName": "nano-spores-8933019524.png",
  "itemCount": 25000,
  "masterable": false,
  "name": "Nanosporen",
  "parents": [
    "Acceltra",
    "Acrid",
    "Akarius",
    "Ancient Protector Specter",
    "Ankyros",
    "Ballistica",
    "Bubonico",
    "Carrier",
    "Cestra",
    "Charger Specter",
    "Clem Clone",
    "Clotra Stim",
    "Cobra & Crane",
    "Control Module",
    "Corrupted Bombard Specter",
    "Corrupted Lancer Specter",
    "Cosmic Specter",
    "Cyanex",
    "Dethcube",
    "Drakgoon",
    "Dual Toxocyst",
    "Ether Daggers",
    "Excalibur Umbra",
    "Fang",
    "Fomorian Disruptor",
    "Fulmin",
    "Furax",
    "Gallium",
    "Genetic Code Template",
    "Glaive",
    "Gunsen",
    "Halikar",
    "Harpak",
    "Hate",
    "Hirudo",
    "Ignis",
    "Incubator Power Core",
    "Infested Catalyst",
    "Kestrel",
    "Kronen",
    "Kunai",
    "Lesion",
    "Mios",
    "Mire",
    "Moa Specter",
    "Morphics",
    "Mutalist Alad V Assassinate",
    "Mutalist Cernos",
    "Mutalist Quanta",
    "Nagantaka",
    "Paracyst",
    "Paris",
    "Patient Zero",
    "Phage",
    "Phantasma",
    "Phase Specter",
    "Plague Keewar",
    "Plague Kripath",
    "Pox",
    "Proboscis Cernos",
    "Pulmonars",
    "Refract Stim",
    "Roller Specter",
    "Sands Of Inaros",
    "Scoliac",
    "Shade",
    "Shield Osprey Specter",
    "Squad Ammo Restore (Large)",
    "Squad Ammo Restore (Small)",
    "Squad Energy Restore (Large)",
    "Squad Energy Restore (Small)",
    "Squad Health Restore (Large)",
    "Squad Health Restore (Medium)",
    "Squad Health Restore (Small)",
    "Squad Shield Restore (Small)",
    "Stalker Specter",
    "Tempered Bapholite",
    "Titan Extractor Prime",
    "Tysis",
    "Umbra Forma",
    "Vectis",
    "Viper",
    "Wyrm",
    "Zakti"
  ],
  "patchlogs": [
    {
      "name": "Prime Resurgence: Hotfix 30.9.3",
      "date": "2021-11-12T21:13:49Z",
      "url": "https://forums.warframe.com/topic/1286448-prime-resurgence-hotfix-3093/",
      "additions": "",
      "changes": "",
      "fixes": "Nano Spores"
    },
    {
      "name": "Buried Debts: Update 24.4.0",
      "date": "2019-03-08T02:01:23Z",
      "url": "https://forums.warframe.com/topic/1067397-buried-debts-update-2440/",
      "additions": "",
      "changes": "Changed Burston building requirements to 600 Ferrite instead of 600 Nano Spores.",
      "fixes": ""
    },
    {
      "name": "Update 15.8.0",
      "date": "2014-12-19T23:27:57Z",
      "url": "https://forums.warframe.com/topic/369665-update-1580/",
      "additions": "",
      "changes": "",
      "fixes": "You will receive a Blueprint from the Lotus for the Fomorian Disruptor. It will require Nano Spores, Cryotic, and Omega Isotopes to craft."
    }
  ],
  "tradable": false,
  "type": "Resource",
  "uniqueName": "/Lotus/Types/Items/MiscItems/Nanospores"
}
"#
}
