use crate::fixture;

fixture! {
    syndicate_mission,
r#"
[
  {
    "id": "1740339093350CetusSyndicate",
    "activation": "2025-02-23T17:01:34.476Z",
    "startString": "-1h 15m 40s",
    "expiry": "2025-02-23T19:31:33.350Z",
    "active": true,
    "syndicate": "Ostrons",
    "syndicateKey": "Ostrons",
    "nodes": [],
    "jobs": [
      {
        "id": "AttritionBountyLib1740339093350",
        "rewardPool": [
          "Vitality",
          "200X Plastids",
          "1,500 Credits Cache",
          "50 Endo",
          "15X Nistlepod",
          "Gara Chassis Blueprint",
          "Point Blank",
          "Intensify",
          "2X Gallium"
        ],
        "type": "Weaken the Grineer Foothold",
        "enemyLevels": [5, 15],
        "standingStages": [490, 490, 490],
        "minMR": 0,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "ReclamationBountyCache1740339093350",
        "rewardPool": [
          "Point Strike",
          "300X Circuits",
          "2,500 Credits Cache",
          "100 Endo",
          "Gara Systems Blueprint",
          "Enhanced Durability",
          "Grim Fury",
          "Aya",
          "2X Orokin Cell"
        ],
        "type": "Find the Hidden Artifact",
        "enemyLevels": [10, 30],
        "standingStages": [670, 670, 670],
        "minMR": 1,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "RescueBountyResc1740339093350",
        "rewardPool": [
          "Augur Pact",
          "Naramon Lens",
          "Zenurik Lens",
          "200 Endo",
          "Gara Neuroptics Blueprint",
          "Vigilante Fervor",
          "Revenant Systems Blueprint",
          "Aya",
          "Gladiator Vice"
        ],
        "type": "Search and Rescue",
        "enemyLevels": [20, 40],
        "standingStages": [570, 570, 570, 830],
        "minMR": 2,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "AssassinateBountyCap1740339093350",
        "rewardPool": [
          "Augur Message",
          "100X Kuva",
          "Naramon Lens",
          "300 Endo",
          "Cetus Wisp",
          "Vigilante Pursuit",
          "Revenant Chassis Blueprint",
          "Aya",
          "Gladiator Finesse"
        ],
        "type": "Capture the New Grineer Commander",
        "enemyLevels": [30, 50],
        "standingStages": [570, 570, 570, 570, 1120],
        "minMR": 3,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "AssassinateBountyAss1740339093350",
        "rewardPool": [
          "5X Breath Of The Eidolon",
          "400 Endo",
          "2X Cetus Wisp",
          "300X Kuva",
          "Aya",
          "Furax Wraith Blueprint",
          "Twirling Spire",
          "Eidolon Lens Blueprint",
          "Revenant Neuroptics Blueprint"
        ],
        "type": "Assassinate the Commander",
        "enemyLevels": [40, 60],
        "standingStages": [710, 710, 710, 710, 1390],
        "minMR": 5,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "AttritionBountySab1740339093350",
        "rewardPool": [
          "5X Breath Of The Eidolon",
          "400 Endo",
          "2X Cetus Wisp",
          "300X Kuva",
          "Aya",
          "Furax Wraith Blueprint",
          "Twirling Spire",
          "Eidolon Lens Blueprint",
          "Revenant Neuroptics Blueprint"
        ],
        "type": "Sabotage the Enemy Supply Lines",
        "enemyLevels": [100, 100],
        "standingStages": [840, 840, 840, 840, 1660],
        "minMR": 10,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "AssassinateBountyAss1740339093350",
        "rewardPool": [
          "Nira's Anguish",
          "Narmer Isoplast",
          "600 Endo",
          "Caliban Neuroptics Blueprint",
          "Amar's Hatred",
          "2X Narmer Isoplast",
          "900 Endo",
          "Boreal's Contempt",
          "Korumm Blueprint",
          "3X Narmer Isoplast",
          "1200 Endo"
        ],
        "type": "Rise and Fall (Narmer)",
        "enemyLevels": [50, 70],
        "standingStages": [840, 840, 840, 840, 1640],
        "minMR": 0,
        "expiry": "2025-02-23T18:41:33.350Z",
        "timeBound": "day"
      }
    ],
    "eta": "1h 14m 18s"
  },
  {
    "id": "1740339093350EntratiLabSyndicate",
    "activation": "2025-02-23T17:01:34.476Z",
    "startString": "-1h 15m 40s",
    "expiry": "2025-02-23T19:31:33.350Z",
    "active": true,
    "syndicate": "Cavia",
    "syndicateKey": "Cavia",
    "nodes": [],
    "jobs": [],
    "eta": "1h 14m 18s"
  },
  {
    "id": "1740339093350EntratiSyndicate",
    "activation": "2025-02-23T17:01:34.476Z",
    "startString": "-1h 15m 35s",
    "expiry": "2025-02-23T19:31:33.350Z",
    "active": true,
    "syndicate": "Entrati",
    "syndicateKey": "Entrati",
    "nodes": [],
    "jobs": [
      {
        "id": "DeimosKeyPiecesBounty1740339093350",
        "rewardPool": [
          "3X 1,500 Credits Cache",
          "150 Endo",
          "15X Lucent Teroglobe",
          "2X 3,000 Credits Cache",
          "250 Endo",
          "Aya",
          "Scintillant"
        ],
        "type": "Salvage",
        "enemyLevels": [5, 15],
        "standingStages": [5, 5, 5],
        "minMR": 0,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "DeimosAreaDefenseBounty1740339093350",
        "rewardPool": [
          "600 Endo",
          "Naramon Lens",
          "Zenurik Lens",
          "Body Count",
          "Scintillant",
          "Aya",
          "Xaku Neuroptics Blueprint",
          "Spring-Loaded Chamber",
          "Repeater Clip",
          "Pressurized Magazine"
        ],
        "type": "Reclaim What's Ours",
        "enemyLevels": [15, 25],
        "standingStages": [10, 10, 10],
        "minMR": 1,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "DeimosEndlessPurifyBounty1740339093350",
        "rewardPool": [
          "Ayatan Amber Star",
          "250 Endo",
          "Carnis Mandible",
          "Jugulus Barbs",
          "Saxum Thorax",
          "500 Endo",
          "Aya",
          "Carnis Carapace",
          "Jugulus Carapace",
          "Saxum Carapace",
          "750 Endo",
          "Carnis Stinger",
          "Jugulus Spines",
          "Saxum Spittle"
        ],
        "type": "Anomaly Retrieval (Endless)",
        "enemyLevels": [25, 30],
        "standingStages": [14, 14, 14],
        "minMR": 5,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "DeimosPurifyBounty1740339093350",
        "rewardPool": [
          "750 Endo",
          "Unairu Lens",
          "Madurai Lens",
          "Hydraulic Crosshairs",
          "5X Fass Residue",
          "Aya",
          "Xaku Systems Blueprint",
          "Laser Sight",
          "Blood Rush",
          "Argon Scope"
        ],
        "type": "Anomaly Retrieval",
        "enemyLevels": [30, 40],
        "standingStages": [16, 16, 16, 24],
        "minMR": 2,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "DeimosAssassinateBounty1740339093350",
        "rewardPool": [
          "Ayatan Amber Star",
          "1000 Endo",
          "750X Kuva",
          "Aya",
          "Xaku Chassis Blueprint",
          "Quassus Blueprint"
        ],
        "type": "Cleanse the Land",
        "enemyLevels": [40, 60],
        "standingStages": [22, 22, 22, 22, 43],
        "minMR": 3,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "DeimosExcavateBounty1740339093350",
        "rewardPool": [
          "Ayatan Amber Star",
          "1000 Endo",
          "750X Kuva",
          "Aya",
          "Xaku Chassis Blueprint",
          "Quassus Blueprint"
        ],
        "type": "Core Samples",
        "enemyLevels": [100, 100],
        "standingStages": [25, 25, 25, 25, 50],
        "minMR": 10,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "1740339093350",
        "rewardPool": [
          "Residual Boils",
          "Necramech Redirection",
          "2X Orokin Orientation Matrix",
          "Meso E6 Relic",
          "Residual Shock",
          "Necramech Steel Fiber",
          "2X Orokin Ballistics Matrix",
          "Neo W2 Relic",
          "Neo G8 Relic",
          "Theorem Contagion",
          "Necramech Thrusters",
          "Damaged Necramech Weapon Barrel",
          "3X Scintillant",
          "2X Orokin Animus Matrix",
          "Axi G14 Relic"
        ],
        "type": "Isolation Vault Chamber B",
        "enemyLevels": [30, 40],
        "standingStages": [2, 2, 2, 4],
        "minMR": 5,
        "isVault": true,
        "locationTag": "ChamberB",
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "1740339093350",
        "rewardPool": [
          "Residual Boils",
          "Necramech Blitz",
          "Arum Spinosa Blueprint",
          "3X Orokin Orientation Matrix",
          "Neo W2 Relic",
          "Residual Shock",
          "Necramech Slipstream",
          "Arum Spinosa Guard",
          "3X Orokin Ballistics Matrix",
          "Neo G8 Relic",
          "Neo C6 Relic",
          "Theorem Contagion",
          "Necramech Seismic Wave",
          "Damaged Necramech Weapon Barrel",
          "Arum Spinosa Rivet",
          "3X Orokin Animus Matrix",
          "Neo T9 Relic"
        ],
        "type": "Isolation Vault Chamber A",
        "enemyLevels": [40, 50],
        "standingStages": [4, 4, 4, 5],
        "minMR": 5,
        "isVault": true,
        "locationTag": "ChamberA",
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "1740339093350",
        "rewardPool": [
          "Residual Boils",
          "Necramech Fury",
          "Sporothrix Blueprint",
          "4X Orokin Orientation Matrix",
          "Meso G8 Relic",
          "Residual Shock",
          "Necramech Stretch",
          "Sporothrix Barrel",
          "4X Orokin Ballistics Matrix",
          "Neo W2 Relic",
          "Neo G8 Relic",
          "Theorem Contagion",
          "Necramech Streamline",
          "Damaged Necramech Weapon Barrel",
          "4X Orokin Animus Matrix",
          "Axi P9 Relic"
        ],
        "type": "Isolation Vault Chamber C",
        "enemyLevels": [50, 60],
        "standingStages": [5, 5, 5, 7],
        "minMR": 5,
        "isVault": true,
        "locationTag": "ChamberC",
        "expiry": "2025-02-23T19:31:33.350Z"
      }
    ],
    "eta": "1h 14m 23s"
  },
  {
    "id": "1740339093350HexSyndicate",
    "activation": "2025-02-23T17:01:34.476Z",
    "startString": "-1h 15m 40s",
    "expiry": "2025-02-23T19:31:33.350Z",
    "active": true,
    "syndicate": "HexSyndicate",
    "syndicateKey": "HexSyndicate",
    "nodes": [],
    "jobs": [],
    "eta": "1h 14m 18s"
  },
  {
    "id": "1740339093350SolarisSyndicate",
    "activation": "2025-02-23T17:01:34.476Z",
    "startString": "-1h 15m 40s",
    "expiry": "2025-02-23T19:31:33.350Z",
    "active": true,
    "syndicate": "Solaris United",
    "syndicateKey": "Solaris United",
    "nodes": [],
    "jobs": [
      {
        "id": "VenusTheftJobExcavation1740339093350",
        "rewardPool": [
          "100X Plastids",
          "1,500 Credits Cache",
          "50 Endo",
          "5X Gorgaricus Spore",
          "Training Debt-Bond",
          "Garuda Chassis Blueprint",
          "5X Tepa Nodule",
          "Aya"
        ],
        "type": "Resource Theft",
        "enemyLevels": [5, 15],
        "standingStages": [490, 490, 490],
        "minMR": 0,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "VenusIntelJobRecovery1740339093350",
        "rewardPool": [
          "15X Gorgaricus Spore",
          "200X Nano Spores",
          "2,500 Credits Cache",
          "100 Endo",
          "2X Shelter Debt-Bond",
          "Garuda Systems Blueprint",
          "Aya",
          "Tek Assault"
        ],
        "type": "Proof of Life",
        "enemyLevels": [10, 30],
        "standingStages": [690, 690, 690],
        "minMR": 1,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "VenusChaosJobAssassinate1740339093350",
        "rewardPool": [
          "200 Endo",
          "Naramon Lens",
          "300X Rubedo",
          "2X Medical Debt-Bond",
          "Garuda Neuroptics Blueprint",
          "Aya",
          "Tek Enhance"
        ],
        "type": "Scorched Earth",
        "enemyLevels": [20, 40],
        "standingStages": [660, 660, 660, 980],
        "minMR": 2,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "VenusChaosJobExcavation1740339093350",
        "rewardPool": [
          "200X Kuva",
          "300 Endo",
          "2X Mutagen Mass",
          "2X Advances Debt-Bond",
          "Aya",
          "Tek Gravity"
        ],
        "type": "Bury Them",
        "enemyLevels": [30, 50],
        "standingStages": [590, 590, 590, 590, 1160],
        "minMR": 3,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "VenusWetworkJobSpy1740339093350",
        "rewardPool": [
          "400 Endo",
          "Neurodes",
          "Orokin Cell",
          "2X Familial Debt-Bond",
          "Aya",
          "Tek Collateral"
        ],
        "type": "Falling Star",
        "enemyLevels": [40, 60],
        "standingStages": [720, 720, 720, 720, 1420],
        "minMR": 5,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "VenusIntelJobResource1740339093350",
        "rewardPool": [
          "400 Endo",
          "Neurodes",
          "Orokin Cell",
          "2X Familial Debt-Bond",
          "Aya",
          "Tek Collateral"
        ],
        "type": "Operational Intelligence",
        "enemyLevels": [100, 100],
        "standingStages": [840, 840, 840, 840, 1660],
        "minMR": 10,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "NarmerVenusCullJobExterminate1740339093350",
        "rewardPool": [
          "Nira's Anguish",
          "Narmer Isoplast",
          "600 Endo",
          "Caliban Neuroptics Blueprint",
          "Amar's Hatred",
          "2X Narmer Isoplast",
          "900 Endo",
          "Boreal's Contempt",
          "Korumm Blueprint",
          "3X Narmer Isoplast",
          "1200 Endo"
        ],
        "type": "Crush the Cult (Narmer)",
        "enemyLevels": [50, 70],
        "standingStages": [770, 770, 770, 770, 1500],
        "minMR": 0,
        "expiry": "2025-02-23T19:31:33.350Z",
        "timeBoound": "night"
      }
    ],
    "eta": "1h 14m 18s"
  },
  {
    "id": "1740339093350ZarimanSyndicate",
    "activation": "2025-02-23T17:01:34.476Z",
    "startString": "-1h 15m 40s",
    "expiry": "2025-02-23T19:31:33.350Z",
    "active": true,
    "syndicate": "The Holdfasts",
    "syndicateKey": "The Holdfasts",
    "nodes": [],
    "jobs": [],
    "eta": "1h 14m 18s"
  },
  {
    "id": "1740416340000ArbitersSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Arbiters of Hexis",
    "syndicateKey": "Arbiters of Hexis",
    "nodes": [
      "Cervantes (Earth)",
      "Unda (Venus)",
      "Gulliver (Phobos)",
      "Olympus (Mars)",
      "Lares (Mercury)",
      "Charybdis (Sedna)",
      "Enceladus (Saturn)"
    ],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000CephalonSudaSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Cephalon Suda",
    "syndicateKey": "Cephalon Suda",
    "nodes": [
      "Ultor (Mars)",
      "Linea (Venus)",
      "Dirus (Deimos)",
      "Cinxia (Ceres)",
      "Caloris (Mercury)",
      "Telesto (Saturn)",
      "Naga (Sedna)"
    ],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000EventSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Operations Syndicate",
    "syndicateKey": "Operations Syndicate",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000KahlSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Kahl's Garrison",
    "syndicateKey": "Kahl's Garrison",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000NecraloidSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Necraloid",
    "syndicateKey": "Necraloid",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000NewLokaSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "New Loka",
    "syndicateKey": "New Loka",
    "nodes": [
      "M Prime (Mercury)",
      "Cambria (Earth)",
      "Ose (Europa)",
      "Draco (Ceres)",
      "Kiliken (Venus)",
      "Numa (Saturn)",
      "Shklovsky (Phobos)"
    ],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000PerrinSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Perrin Sequence",
    "syndicateKey": "Perrin Sequence",
    "nodes": [
      "Ares (Mars)",
      "Odin (Mercury)",
      "Cassini (Saturn)",
      "Cytherean (Venus)",
      "Ker (Ceres)",
      "Aten (Void)",
      "Pacific (Earth)"
    ],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000QuillsSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Quills",
    "syndicateKey": "Quills",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000RadioLegion2Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "The Emissary",
    "syndicateKey": "The Emissary",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000RadioLegion3Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Glassmaker",
    "syndicateKey": "Glassmaker",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000RadioLegionIntermission10Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "RadioLegionIntermission10Syndicate",
    "syndicateKey": "RadioLegionIntermission10Syndicate",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000RadioLegionIntermission11Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "RadioLegionIntermission11Syndicate",
    "syndicateKey": "RadioLegionIntermission11Syndicate",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000RadioLegionIntermission12Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "RadioLegionIntermission12Syndicate",
    "syndicateKey": "RadioLegionIntermission12Syndicate",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000RadioLegionIntermission2Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Intermission II",
    "syndicateKey": "Intermission II",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000RadioLegionIntermission3Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Intermission III",
    "syndicateKey": "Intermission III",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000RadioLegionIntermission4Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Nora's Choice",
    "syndicateKey": "Nora's Choice",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000RadioLegionIntermission5Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Nora's Mix Volume 1",
    "syndicateKey": "Nora's Mix Volume 1",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000RadioLegionIntermission6Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Nora's Mix Volume 2",
    "syndicateKey": "Nora's Mix Volume 2",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000RadioLegionIntermission7Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "RadioLegionIntermission7Syndicate",
    "syndicateKey": "RadioLegionIntermission7Syndicate",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000RadioLegionIntermission8Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "RadioLegionIntermission8Syndicate",
    "syndicateKey": "RadioLegionIntermission8Syndicate",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000RadioLegionIntermission9Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "RadioLegionIntermission9Syndicate",
    "syndicateKey": "RadioLegionIntermission9Syndicate",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000RadioLegionIntermissionSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Intermission",
    "syndicateKey": "Intermission",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000RadioLegionSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "The Wolf of Saturn Six",
    "syndicateKey": "The Wolf of Saturn Six",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000RedVeilSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Red Veil",
    "syndicateKey": "Red Veil",
    "nodes": [
      "Augustus (Mars)",
      "Thebe (Jupiter)",
      "Phlegyas (Deimos)",
      "V Prime (Venus)",
      "Ukko (Void)",
      "Pantheon (Mercury)",
      "Horend (Deimos)"
    ],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000SteelMeridianSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Steel Meridian",
    "syndicateKey": "Steel Meridian",
    "nodes": [
      "Ara (Mars)",
      "Boethius (Mercury)",
      "Ananke (Jupiter)",
      "Venera (Venus)",
      "Hepit (Void)",
      "Saxis (Eris)",
      "Calypso (Saturn)"
    ],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000VentKidsSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Vent Kids",
    "syndicateKey": "Vent Kids",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  },
  {
    "id": "1740416340000VoxSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 12s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Vox Solaris",
    "syndicateKey": "Vox Solaris",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 44s"
  }
]
"#
---
r#"
[
  {
    "id": "1740339093350CetusSyndicate",
    "activation": "2025-02-23T17:01:34.476Z",
    "startString": "-1h 16m 17s",
    "expiry": "2025-02-23T19:31:33.350Z",
    "active": true,
    "syndicate": "Ostrons",
    "syndicateKey": "Ostrons",
    "nodes": [],
    "jobs": [
      {
        "id": "AttritionBountyLib1740339093350",
        "rewardPool": [
          "Vitality",
          "200X Plastids",
          "1,500 Credits Cache",
          "50 Endo",
          "15X Nistlepod",
          "Gara Chassis Blueprint",
          "Point Blank",
          "Intensify",
          "2X Gallium"
        ],
        "type": "Weaken the Grineer Foothold",
        "enemyLevels": [5, 15],
        "standingStages": [490, 490, 490],
        "minMR": 0,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "ReclamationBountyCache1740339093350",
        "rewardPool": [
          "Point Strike",
          "300X Circuits",
          "2,500 Credits Cache",
          "100 Endo",
          "Gara Systems Blueprint",
          "Enhanced Durability",
          "Grim Fury",
          "Aya",
          "2X Orokin Cell"
        ],
        "type": "Find the Hidden Artifact",
        "enemyLevels": [10, 30],
        "standingStages": [670, 670, 670],
        "minMR": 1,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "RescueBountyResc1740339093350",
        "rewardPool": [
          "Augur Pact",
          "Naramon Lens",
          "Zenurik Lens",
          "200 Endo",
          "Gara Neuroptics Blueprint",
          "Vigilante Fervor",
          "Revenant Systems Blueprint",
          "Aya",
          "Gladiator Vice"
        ],
        "type": "Search and Rescue",
        "enemyLevels": [20, 40],
        "standingStages": [570, 570, 570, 830],
        "minMR": 2,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "AssassinateBountyCap1740339093350",
        "rewardPool": [
          "Augur Message",
          "100X Kuva",
          "Naramon Lens",
          "300 Endo",
          "Cetus Wisp",
          "Vigilante Pursuit",
          "Revenant Chassis Blueprint",
          "Aya",
          "Gladiator Finesse"
        ],
        "type": "Capture the New Grineer Commander",
        "enemyLevels": [30, 50],
        "standingStages": [570, 570, 570, 570, 1120],
        "minMR": 3,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "AssassinateBountyAss1740339093350",
        "rewardPool": [
          "5X Breath Of The Eidolon",
          "400 Endo",
          "2X Cetus Wisp",
          "300X Kuva",
          "Aya",
          "Furax Wraith Blueprint",
          "Twirling Spire",
          "Eidolon Lens Blueprint",
          "Revenant Neuroptics Blueprint"
        ],
        "type": "Assassinate the Commander",
        "enemyLevels": [40, 60],
        "standingStages": [710, 710, 710, 710, 1390],
        "minMR": 5,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "AttritionBountySab1740339093350",
        "rewardPool": [
          "5X Breath Of The Eidolon",
          "400 Endo",
          "2X Cetus Wisp",
          "300X Kuva",
          "Aya",
          "Furax Wraith Blueprint",
          "Twirling Spire",
          "Eidolon Lens Blueprint",
          "Revenant Neuroptics Blueprint"
        ],
        "type": "Sabotage the Enemy Supply Lines",
        "enemyLevels": [100, 100],
        "standingStages": [840, 840, 840, 840, 1660],
        "minMR": 10,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "AssassinateBountyAss1740339093350",
        "rewardPool": [
          "Nira's Anguish",
          "Narmer Isoplast",
          "600 Endo",
          "Caliban Neuroptics Blueprint",
          "Amar's Hatred",
          "2X Narmer Isoplast",
          "900 Endo",
          "Boreal's Contempt",
          "Korumm Blueprint",
          "3X Narmer Isoplast",
          "1200 Endo"
        ],
        "type": "Rise and Fall (Narmer)",
        "enemyLevels": [50, 70],
        "standingStages": [840, 840, 840, 840, 1640],
        "minMR": 0,
        "expiry": "2025-02-23T18:41:33.350Z",
        "timeBound": "day"
      }
    ],
    "eta": "1h 13m 41s"
  },
  {
    "id": "1740339093350EntratiLabSyndicate",
    "activation": "2025-02-23T17:01:34.476Z",
    "startString": "-1h 16m 17s",
    "expiry": "2025-02-23T19:31:33.350Z",
    "active": true,
    "syndicate": "Cavia",
    "syndicateKey": "Cavia",
    "nodes": [],
    "jobs": [],
    "eta": "1h 13m 41s"
  },
  {
    "id": "1740339093350EntratiSyndicate",
    "activation": "2025-02-23T17:01:34.476Z",
    "startString": "-1h 16m 15s",
    "expiry": "2025-02-23T19:31:33.350Z",
    "active": true,
    "syndicate": "英择谛",
    "syndicateKey": "Entrati",
    "nodes": [],
    "jobs": [
      {
        "id": "DeimosKeyPiecesBounty1740339093350",
        "rewardPool": [
          "3X 1,500 Credits Cache",
          "150 Endo",
          "15X Lucent Teroglobe",
          "2X 3,000 Credits Cache",
          "250 Endo",
          "Aya",
          "Scintillant"
        ],
        "type": "Salvage",
        "enemyLevels": [5, 15],
        "standingStages": [5, 5, 5],
        "minMR": 0,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "DeimosAreaDefenseBounty1740339093350",
        "rewardPool": [
          "600 Endo",
          "Naramon Lens",
          "Zenurik Lens",
          "Body Count",
          "Scintillant",
          "Aya",
          "Xaku Neuroptics Blueprint",
          "Spring-Loaded Chamber",
          "Repeater Clip",
          "Pressurized Magazine"
        ],
        "type": "Reclaim What's Ours",
        "enemyLevels": [15, 25],
        "standingStages": [10, 10, 10],
        "minMR": 1,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "DeimosEndlessPurifyBounty1740339093350",
        "rewardPool": [
          "Ayatan Amber Star",
          "250 Endo",
          "Carnis Mandible",
          "Jugulus Barbs",
          "Saxum Thorax",
          "500 Endo",
          "Aya",
          "Carnis Carapace",
          "Jugulus Carapace",
          "Saxum Carapace",
          "750 Endo",
          "Carnis Stinger",
          "Jugulus Spines",
          "Saxum Spittle"
        ],
        "type": "Anomaly Retrieval (Endless)",
        "enemyLevels": [25, 30],
        "standingStages": [14, 14, 14],
        "minMR": 5,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "DeimosPurifyBounty1740339093350",
        "rewardPool": [
          "750 Endo",
          "Unairu Lens",
          "Madurai Lens",
          "Hydraulic Crosshairs",
          "5X Fass Residue",
          "Aya",
          "Xaku Systems Blueprint",
          "Laser Sight",
          "Blood Rush",
          "Argon Scope"
        ],
        "type": "Anomaly Retrieval",
        "enemyLevels": [30, 40],
        "standingStages": [16, 16, 16, 24],
        "minMR": 2,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "DeimosAssassinateBounty1740339093350",
        "rewardPool": [
          "Ayatan Amber Star",
          "1000 Endo",
          "750X Kuva",
          "Aya",
          "Xaku Chassis Blueprint",
          "Quassus Blueprint"
        ],
        "type": "Cleanse the Land",
        "enemyLevels": [40, 60],
        "standingStages": [22, 22, 22, 22, 43],
        "minMR": 3,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "DeimosExcavateBounty1740339093350",
        "rewardPool": [
          "Ayatan Amber Star",
          "1000 Endo",
          "750X Kuva",
          "Aya",
          "Xaku Chassis Blueprint",
          "Quassus Blueprint"
        ],
        "type": "Core Samples",
        "enemyLevels": [100, 100],
        "standingStages": [25, 25, 25, 25, 50],
        "minMR": 10,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "1740339093350",
        "rewardPool": [
          "Residual Boils",
          "Necramech Redirection",
          "2X Orokin Orientation Matrix",
          "Meso E6 Relic",
          "Residual Shock",
          "Necramech Steel Fiber",
          "2X Orokin Ballistics Matrix",
          "Neo W2 Relic",
          "Neo G8 Relic",
          "Theorem Contagion",
          "Necramech Thrusters",
          "Damaged Necramech Weapon Barrel",
          "3X Scintillant",
          "2X Orokin Animus Matrix",
          "Axi G14 Relic"
        ],
        "type": "Isolation Vault Chamber B",
        "enemyLevels": [30, 40],
        "standingStages": [2, 2, 2, 4],
        "minMR": 5,
        "isVault": true,
        "locationTag": "ChamberB",
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "1740339093350",
        "rewardPool": [
          "Residual Boils",
          "Necramech Blitz",
          "Arum Spinosa Blueprint",
          "3X Orokin Orientation Matrix",
          "Neo W2 Relic",
          "Residual Shock",
          "Necramech Slipstream",
          "Arum Spinosa Guard",
          "3X Orokin Ballistics Matrix",
          "Neo G8 Relic",
          "Neo C6 Relic",
          "Theorem Contagion",
          "Necramech Seismic Wave",
          "Damaged Necramech Weapon Barrel",
          "Arum Spinosa Rivet",
          "3X Orokin Animus Matrix",
          "Neo T9 Relic"
        ],
        "type": "Isolation Vault Chamber A",
        "enemyLevels": [40, 50],
        "standingStages": [4, 4, 4, 5],
        "minMR": 5,
        "isVault": true,
        "locationTag": "ChamberA",
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "1740339093350",
        "rewardPool": [
          "Residual Boils",
          "Necramech Fury",
          "Sporothrix Blueprint",
          "4X Orokin Orientation Matrix",
          "Meso G8 Relic",
          "Residual Shock",
          "Necramech Stretch",
          "Sporothrix Barrel",
          "4X Orokin Ballistics Matrix",
          "Neo W2 Relic",
          "Neo G8 Relic",
          "Theorem Contagion",
          "Necramech Streamline",
          "Damaged Necramech Weapon Barrel",
          "4X Orokin Animus Matrix",
          "Axi P9 Relic"
        ],
        "type": "Isolation Vault Chamber C",
        "enemyLevels": [50, 60],
        "standingStages": [5, 5, 5, 7],
        "minMR": 5,
        "isVault": true,
        "locationTag": "ChamberC",
        "expiry": "2025-02-23T19:31:33.350Z"
      }
    ],
    "eta": "1h 13m 43s"
  },
  {
    "id": "1740339093350HexSyndicate",
    "activation": "2025-02-23T17:01:34.476Z",
    "startString": "-1h 16m 17s",
    "expiry": "2025-02-23T19:31:33.350Z",
    "active": true,
    "syndicate": "HexSyndicate",
    "syndicateKey": "HexSyndicate",
    "nodes": [],
    "jobs": [],
    "eta": "1h 13m 41s"
  },
  {
    "id": "1740339093350SolarisSyndicate",
    "activation": "2025-02-23T17:01:34.476Z",
    "startString": "-1h 16m 17s",
    "expiry": "2025-02-23T19:31:33.350Z",
    "active": true,
    "syndicate": "索拉里斯联盟",
    "syndicateKey": "Solaris United",
    "nodes": [],
    "jobs": [
      {
        "id": "VenusTheftJobExcavation1740339093350",
        "rewardPool": [
          "100X Plastids",
          "1,500 Credits Cache",
          "50 Endo",
          "5X Gorgaricus Spore",
          "Training Debt-Bond",
          "Garuda Chassis Blueprint",
          "5X Tepa Nodule",
          "Aya"
        ],
        "type": "Resource Theft",
        "enemyLevels": [5, 15],
        "standingStages": [490, 490, 490],
        "minMR": 0,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "VenusIntelJobRecovery1740339093350",
        "rewardPool": [
          "15X Gorgaricus Spore",
          "200X Nano Spores",
          "2,500 Credits Cache",
          "100 Endo",
          "2X Shelter Debt-Bond",
          "Garuda Systems Blueprint",
          "Aya",
          "Tek Assault"
        ],
        "type": "Proof of Life",
        "enemyLevels": [10, 30],
        "standingStages": [690, 690, 690],
        "minMR": 1,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "VenusChaosJobAssassinate1740339093350",
        "rewardPool": [
          "200 Endo",
          "Naramon Lens",
          "300X Rubedo",
          "2X Medical Debt-Bond",
          "Garuda Neuroptics Blueprint",
          "Aya",
          "Tek Enhance"
        ],
        "type": "Scorched Earth",
        "enemyLevels": [20, 40],
        "standingStages": [660, 660, 660, 980],
        "minMR": 2,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "VenusChaosJobExcavation1740339093350",
        "rewardPool": [
          "200X Kuva",
          "300 Endo",
          "2X Mutagen Mass",
          "2X Advances Debt-Bond",
          "Aya",
          "Tek Gravity"
        ],
        "type": "Bury Them",
        "enemyLevels": [30, 50],
        "standingStages": [590, 590, 590, 590, 1160],
        "minMR": 3,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "VenusWetworkJobSpy1740339093350",
        "rewardPool": [
          "400 Endo",
          "Neurodes",
          "Orokin Cell",
          "2X Familial Debt-Bond",
          "Aya",
          "Tek Collateral"
        ],
        "type": "Falling Star",
        "enemyLevels": [40, 60],
        "standingStages": [720, 720, 720, 720, 1420],
        "minMR": 5,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "VenusIntelJobResource1740339093350",
        "rewardPool": [
          "400 Endo",
          "Neurodes",
          "Orokin Cell",
          "2X Familial Debt-Bond",
          "Aya",
          "Tek Collateral"
        ],
        "type": "Operational Intelligence",
        "enemyLevels": [100, 100],
        "standingStages": [840, 840, 840, 840, 1660],
        "minMR": 10,
        "expiry": "2025-02-23T19:31:33.350Z"
      },
      {
        "id": "NarmerVenusCullJobExterminate1740339093350",
        "rewardPool": [
          "Nira's Anguish",
          "Narmer Isoplast",
          "600 Endo",
          "Caliban Neuroptics Blueprint",
          "Amar's Hatred",
          "2X Narmer Isoplast",
          "900 Endo",
          "Boreal's Contempt",
          "Korumm Blueprint",
          "3X Narmer Isoplast",
          "1200 Endo"
        ],
        "type": "Crush the Cult (Narmer)",
        "enemyLevels": [50, 70],
        "standingStages": [770, 770, 770, 770, 1500],
        "minMR": 0,
        "expiry": "2025-02-23T19:31:33.350Z",
        "timeBoound": "night"
      }
    ],
    "eta": "1h 13m 41s"
  },
  {
    "id": "1740339093350ZarimanSyndicate",
    "activation": "2025-02-23T17:01:34.476Z",
    "startString": "-1h 16m 17s",
    "expiry": "2025-02-23T19:31:33.350Z",
    "active": true,
    "syndicate": "坚守者",
    "syndicateKey": "The Holdfasts",
    "nodes": [],
    "jobs": [],
    "eta": "1h 13m 41s"
  },
  {
    "id": "1740416340000ArbitersSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "均衡仲裁者",
    "syndicateKey": "Arbiters of Hexis",
    "nodes": [
      "Cervantes (Earth)",
      "Unda (Venus)",
      "Gulliver (Phobos)",
      "Olympus (Mars)",
      "Lares (Mercury)",
      "Charybdis (Sedna)",
      "Enceladus (Saturn)"
    ],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000CephalonSudaSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "中樞蘇達",
    "syndicateKey": "Cephalon Suda",
    "nodes": [
      "Ultor (Mars)",
      "Linea (Venus)",
      "Dirus (Deimos)",
      "Cinxia (Ceres)",
      "Caloris (Mercury)",
      "Telesto (Saturn)",
      "Naga (Sedna)"
    ],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000EventSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "集團",
    "syndicateKey": "Operations Syndicate",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000KahlSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "卡尔驻军",
    "syndicateKey": "Kahl's Garrison",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000NecraloidSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "殁世械灵",
    "syndicateKey": "Necraloid",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000NewLokaSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "新世間",
    "syndicateKey": "New Loka",
    "nodes": [
      "M Prime (Mercury)",
      "Cambria (Earth)",
      "Ose (Europa)",
      "Draco (Ceres)",
      "Kiliken (Venus)",
      "Numa (Saturn)",
      "Shklovsky (Phobos)"
    ],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000PerrinSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "佩蘭數列",
    "syndicateKey": "Perrin Sequence",
    "nodes": [
      "Ares (Mars)",
      "Odin (Mercury)",
      "Cassini (Saturn)",
      "Cytherean (Venus)",
      "Ker (Ceres)",
      "Aten (Void)",
      "Pacific (Earth)"
    ],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000QuillsSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "夜羽",
    "syndicateKey": "Quills",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000RadioLegion2Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "使徒",
    "syndicateKey": "The Emissary",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000RadioLegion3Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "玻璃匠",
    "syndicateKey": "Glassmaker",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000RadioLegionIntermission10Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "RadioLegionIntermission10Syndicate",
    "syndicateKey": "RadioLegionIntermission10Syndicate",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000RadioLegionIntermission11Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "RadioLegionIntermission11Syndicate",
    "syndicateKey": "RadioLegionIntermission11Syndicate",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000RadioLegionIntermission12Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "RadioLegionIntermission12Syndicate",
    "syndicateKey": "RadioLegionIntermission12Syndicate",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000RadioLegionIntermission2Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "间歇II",
    "syndicateKey": "Intermission II",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000RadioLegionIntermission3Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "间歇III",
    "syndicateKey": "Intermission III",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000RadioLegionIntermission4Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Nora 的精选",
    "syndicateKey": "Nora's Choice",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000RadioLegionIntermission5Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Nora 的混选 Vol. 1",
    "syndicateKey": "Nora's Mix Volume 1",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000RadioLegionIntermission6Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "Nora 的混选 Vol. 2",
    "syndicateKey": "Nora's Mix Volume 2",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000RadioLegionIntermission7Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "RadioLegionIntermission7Syndicate",
    "syndicateKey": "RadioLegionIntermission7Syndicate",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000RadioLegionIntermission8Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "RadioLegionIntermission8Syndicate",
    "syndicateKey": "RadioLegionIntermission8Syndicate",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000RadioLegionIntermission9Syndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "RadioLegionIntermission9Syndicate",
    "syndicateKey": "RadioLegionIntermission9Syndicate",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000RadioLegionIntermissionSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "间歇",
    "syndicateKey": "Intermission",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000RadioLegionSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "土星六号之狼",
    "syndicateKey": "The Wolf of Saturn Six",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000RedVeilSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "血色面紗",
    "syndicateKey": "Red Veil",
    "nodes": [
      "Augustus (Mars)",
      "Thebe (Jupiter)",
      "Phlegyas (Deimos)",
      "V Prime (Venus)",
      "Ukko (Void)",
      "Pantheon (Mercury)",
      "Horend (Deimos)"
    ],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000SteelMeridianSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "鋼鐵防線",
    "syndicateKey": "Steel Meridian",
    "nodes": [
      "Ara (Mars)",
      "Boethius (Mercury)",
      "Ananke (Jupiter)",
      "Venera (Venus)",
      "Hepit (Void)",
      "Saxis (Eris)",
      "Calypso (Saturn)"
    ],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000VentKidsSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "通風小子",
    "syndicateKey": "Vent Kids",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  },
  {
    "id": "1740416340000VoxSyndicate",
    "activation": "2025-02-23T16:59:02.986Z",
    "startString": "-1h 18m 48s",
    "expiry": "2025-02-24T16:59:00.000Z",
    "active": true,
    "syndicate": "索里拉斯之聲",
    "syndicateKey": "Vox Solaris",
    "nodes": [],
    "jobs": [],
    "eta": "22h 41m 8s"
  }
]
"#
}
