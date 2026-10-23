use serde::Deserialize;
use warframe_macros::model;

macro_rules! unordered_pattern {
    ($a:pat, $b:pat) => {
        ($a, $b) | ($b, $a)
    };
}

/// A damage type in Warframe
#[derive(
    Clone, Debug, PartialEq, Eq, derive_more::From, derive_more::TryInto, Hash, derive_more::Display,
)]
pub enum DamageType {
    /// Physical Damage
    Physical(PhysicalDamage),
    /// Primary Elemental Damage
    Elemental(ElementalDamage),
    /// Secondary Elemental Damage
    Combined(CombinedElementalDamage),
    /// An unrecognized damage type
    Unknown(String),
}

impl<'de> Deserialize<'de> for DamageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value =
            serde_json::Value::deserialize(deserializer).map_err(serde::de::Error::custom)?;

        if let Ok(v) = PhysicalDamage::deserialize(&value) {
            return Ok(DamageType::Physical(v));
        }
        if let Ok(v) = ElementalDamage::deserialize(&value) {
            return Ok(DamageType::Elemental(v));
        }
        if let Ok(v) = CombinedElementalDamage::deserialize(&value) {
            return Ok(DamageType::Combined(v));
        }

        match value.as_str() {
            Some(s) => Ok(DamageType::Unknown(s.to_owned())),
            None => Ok(DamageType::Unknown(value.to_string())),
        }
    }
}

/// Physical damage types
#[model]
#[serde(rename_all = "lowercase")]
pub enum PhysicalDamage {
    /// Impact
    Impact,
    /// Puncture
    Puncture,
    /// Slash
    Slash,
}

/// Primary elemental damage types
#[model]
#[serde(rename_all = "lowercase")]
pub enum ElementalDamage {
    /// Heat
    Heat,
    /// Cold
    Cold,
    /// Electricity
    Electricity,
    /// Toxin
    Toxin,
    /// Void
    Void,
    /// Tau
    Tau,
}

impl ElementalDamage {
    /// Combines two Primary Elements into their Combined Element.
    ///
    /// Returns [None] if both `a` and `b` are equal.
    #[must_use]
    pub const fn combine(a: Self, b: Self) -> Option<CombinedElementalDamage> {
        use CombinedElementalDamage::{
            Blast,
            Corrosive,
            Gas,
            Magnetic,
            Radiation,
            Viral,
        };
        use ElementalDamage::{
            Cold,
            Electricity,
            Heat,
            Toxin,
        };

        let combined_element = match (a, b) {
            unordered_pattern!(Heat, Cold) => Blast,
            unordered_pattern!(Heat, Electricity) => Radiation,
            unordered_pattern!(Heat, Toxin) => Gas,
            unordered_pattern!(Toxin, Cold) => Viral,
            unordered_pattern!(Toxin, Electricity) => Corrosive,
            unordered_pattern!(Cold, Electricity) => Magnetic,
            _ => return None,
        };

        Some(combined_element)
    }
}

/// Combined (secondary) elemental damage types
#[model]
#[serde(rename_all = "lowercase")]
pub enum CombinedElementalDamage {
    /// Radiation
    Radiation,
    /// Blast
    Blast,
    /// Viral
    Viral,
    /// Corrosive
    Corrosive,
    /// Gas
    Gas,
    /// Magnetic
    Magnetic,
}

impl CombinedElementalDamage {
    /// Breaks down a combined element into both of its
    /// [`ElementalDamage`](crate::worldstate::ElementalDamage) components.
    ///
    /// The order in which they are returned follows classic HCET.
    #[must_use]
    pub const fn break_down(self) -> (ElementalDamage, ElementalDamage) {
        use CombinedElementalDamage::{
            Blast,
            Corrosive,
            Gas,
            Magnetic,
            Radiation,
            Viral,
        };
        use ElementalDamage::{
            Cold,
            Electricity,
            Heat,
            Toxin,
        };

        match self {
            Radiation => (Heat, Electricity),
            Blast => (Heat, Cold),
            Viral => (Cold, Toxin),
            Corrosive => (Electricity, Toxin),
            Gas => (Heat, Toxin),
            Magnetic => (Cold, Electricity),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_combine() {
        let cold = ElementalDamage::Cold;
        let electricity = ElementalDamage::Electricity;

        let combined: Option<CombinedElementalDamage> = ElementalDamage::combine(cold, electricity);

        assert_eq!(combined.unwrap(), CombinedElementalDamage::Magnetic);
    }

    #[test]
    fn test_break_down() {
        let corrosive = CombinedElementalDamage::Corrosive;

        assert_eq!(
            corrosive.break_down(),
            (ElementalDamage::Electricity, ElementalDamage::Toxin)
        );
    }
}
