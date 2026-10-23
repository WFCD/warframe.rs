#![allow(clippy::doc_markdown)]
use warframe_macros::model;

use super::damage_type::{
    CombinedElementalDamage,
    DamageType,
    ElementalDamage,
    PhysicalDamage,
};

/// An enemy faction in Warframe
#[model]
pub enum EnemyFaction {
    /// Orokin
    Orokin,
    /// Corrupted
    Corrupted,
    /// Infested
    #[serde(alias = "Infestation")]
    Infested,
    /// Corpus
    Corpus,
    /// Grineer
    Grineer,
    /// Narmer
    Narmer,
    /// Murmur
    #[serde(rename(deserialize = "The Murmur"))]
    Murmur,
    /// Man in the Wall
    #[serde(rename(deserialize = "Man in the Wall"))]
    ManInTheWall,
    /// Scaldra - Grineer-derived faction from 1999 Holvenia
    Scaldra,
    /// Techrot - Corpus-derived faction from 1999 Holvenia
    Techrot,
    /// Sentient
    Sentient,
    /// Anarchs - Dax separatists in the Tau system
    Anarchs,
    /// Stalker and Acolytes
    Stalker,
    /// Wild creatures
    Wild,
}

/// A syndicate faction in Warframe
#[model]
#[non_exhaustive]
pub enum SyndicateFaction {
    /// Arbiters of Hexis
    #[serde(rename = "Arbiters of Hexis")]
    ArbitersOfHexis,
    /// Cephalon Suda
    #[serde(rename = "Cephalon Suda")]
    CephalonSuda,
    /// Assassins
    #[serde(rename = "Assassins")]
    Assassins,
    /// Nightwave
    #[serde(rename = "Nightwave")]
    Nightwave,
    /// Ostrons
    #[serde(rename = "Ostrons")]
    Ostrons,
    /// Vox Solaris
    #[serde(rename = "Vox Solaris")]
    VoxSolaris,
    /// Solaris United
    #[serde(rename = "Solaris United")]
    SolarisUnited,
    /// Perrin Sequence
    #[serde(rename = "Perrin Sequence")]
    PerrinSequence,
    /// Steel Meridian
    #[serde(rename = "Steel Meridian")]
    SteelMeridian,
    /// Red Veil
    #[serde(rename = "Red Veil")]
    RedVeil,
    /// New Loka
    #[serde(rename = "New Loka")]
    NewLoka,
    /// The Holdfasts
    #[serde(rename = "The Holdfasts")]
    Holdfasts,
    /// Entrati
    Entrati,
    /// Cavia
    #[serde(rename = "EntratiLabSyndicate", alias = "Cavia")]
    Cavia,
    /// Vent Kids / Operations Syndicate
    #[serde(rename = "Operations Syndicate", alias = "Vent Kids")]
    VentKids,
    /// Kahl's Garrison
    #[serde(rename = "Kahl's Garrison")]
    KahlsGarrison,
    /// Necraloid
    Necraloid,
    /// The Hex
    #[serde(rename = "HexSyndicate", alias = "The Hex")]
    TheHex,
    /// Quills
    #[serde(rename = "Quills")]
    Quills,
}

/// Tenno-aligned factions
#[model]
pub enum TennoFaction {
    /// Tenno
    Tenno,
    /// Crossfire, which is Faction-less
    Crossfire,
}

/// A faction in Warframe, encompassing enemy factions, syndicates, and Tenno-aligned groups
#[derive(
    Debug,
    PartialEq,
    PartialOrd,
    Clone,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::From,
    derive_more::TryInto,
)]
pub enum Faction {
    /// An enemy faction
    Enemy(EnemyFaction),
    /// A syndicate
    Syndicate(SyndicateFaction),
    /// Tenno-aligned
    Tenno(TennoFaction),
    /// An unrecognized faction
    Unknown(String),
}

impl<'de> serde::Deserialize<'de> for Faction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value =
            serde_json::Value::deserialize(deserializer).map_err(serde::de::Error::custom)?;

        if let Ok(v) = EnemyFaction::deserialize(&value) {
            return Ok(Faction::Enemy(v));
        }
        if let Ok(v) = TennoFaction::deserialize(&value) {
            return Ok(Faction::Tenno(v));
        }
        if let Ok(v) = SyndicateFaction::deserialize(&value) {
            return Ok(Faction::Syndicate(v));
        }

        match value.as_str() {
            Some(s) => Ok(Faction::Unknown(s.to_owned())),
            None => Ok(Faction::Unknown(value.to_string())),
        }
    }
}

impl EnemyFaction {
    /// Returns the damage types this enemy faction is vulnerable to.
    #[must_use]
    pub fn vulnerable_to(&self) -> Vec<DamageType> {
        use CombinedElementalDamage::{
            Corrosive,
            Magnetic,
            Radiation,
            Viral,
        };
        use DamageType::{
            Combined,
            Elemental,
            Physical,
        };
        use ElementalDamage::{
            Electricity,
            Heat,
            Toxin,
        };
        use EnemyFaction::{
            Corpus,
            Corrupted,
            Grineer,
            Infested,
            Murmur,
            Narmer,
            Orokin,
        };
        use PhysicalDamage::{
            Impact,
            Puncture,
            Slash,
        };

        match self {
            Orokin | Corrupted => vec![Physical(Puncture), Combined(Viral)],
            Infested => vec![Physical(Slash), Elemental(Heat)],
            Corpus => vec![Physical(Puncture), Combined(Magnetic)],
            Grineer => vec![Physical(Impact), Combined(Corrosive)],
            Narmer => vec![Physical(Slash), Elemental(Toxin)],
            Murmur => vec![Elemental(Electricity), Combined(Radiation)],
            _ => vec![],
        }
    }

    /// Returns the damage type this enemy faction is resistant to, if any.
    #[must_use]
    pub fn resistant_to(&self) -> Option<DamageType> {
        use CombinedElementalDamage::{
            Magnetic,
            Radiation,
            Viral,
        };
        use DamageType::Combined;
        use EnemyFaction::{
            Corrupted,
            Murmur,
            Narmer,
            Orokin,
        };

        match self {
            Orokin | Corrupted => Some(Combined(Radiation)),
            Narmer => Some(Combined(Magnetic)),
            Murmur => Some(Combined(Viral)),
            _ => None,
        }
    }
}

impl Faction {
    /// Returns the damage types this faction is vulnerable to
    #[must_use]
    pub fn vulnerable_to(&self) -> Vec<DamageType> {
        match self {
            Faction::Enemy(e) => e.vulnerable_to(),
            _ => vec![],
        }
    }

    /// Returns the damage type this faction is resistant to
    #[must_use]
    pub fn resistant_to(&self) -> Option<DamageType> {
        match self {
            Faction::Enemy(e) => e.resistant_to(),
            _ => None,
        }
    }
}
