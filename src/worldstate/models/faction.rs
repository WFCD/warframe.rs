use super::damage_type::{
    CombinedElementalDamage,
    DamageType,
    ElementalDamage,
    PhysicalDamage,
};

#[derive(Debug, serde::Deserialize, PartialEq, PartialOrd, Clone, Eq, Copy)]
/// A Faction in Warframe
pub enum Faction {
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
    /// Tenno
    Tenno,
    /// Narmer
    Narmer,
    /// Crossfire, which is Faction-less
    Crossfire,
    /// Murmur
    #[serde(rename(deserialize = "The Murmur"))]
    Murmur,
    /// This is a placeholder faction. For example, it was used during the Jade event
    #[serde(rename(deserialize = "Man in the Wall"))]
    ManInTheWall,
}

impl crate::ws::VariantDocumentation for Faction {
    fn docs(&self) -> &'static str {
        match self {
            Faction::Orokin => "Orokin",
            Faction::Corrupted => "Corrupted",
            Faction::Infested => "Infested",
            Faction::Corpus => "Corpus",
            Faction::Grineer => "Grineer",
            Faction::Tenno => "Tenno",
            Faction::Narmer => "Narmer",
            Faction::Crossfire => "Crossfire, which is Faction-less",
            Faction::ManInTheWall => {
                "This is a placeholder faction. For example, it was used during"
            }
            Faction::Murmur => "Murmur",
        }
    }
}
impl crate::ws::TypeDocumentation for Faction {
    fn docs() -> &'static str {
        "A Faction in Warframe"
    }
}

impl Faction {
    #[must_use]
    pub fn vulnerable_to(self) -> Vec<super::DamageType> {
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
        use Faction::{
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

    #[must_use]
    pub fn resistant_to(self) -> Option<DamageType> {
        use CombinedElementalDamage::{
            Magnetic,
            Radiation,
            Viral,
        };
        use DamageType::Combined;
        use Faction::{
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
