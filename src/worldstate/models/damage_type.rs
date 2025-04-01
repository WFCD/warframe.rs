use serde::Deserialize;
use serde_with::rust::deserialize_ignore_any;

macro_rules! unordered_pattern {
    ($a:pat, $b:pat) => {
        ($a, $b) | ($b, $a)
    };
}

#[derive(
    Clone, Copy, Debug, Deserialize, PartialEq, Eq, derive_more::From, Hash, derive_more::Display,
)]
#[serde(untagged)]
pub enum DamageType {
    // Physical Damage
    Physical(PhysicalDamage),
    // Primary Elemental Damage
    Elemental(ElementalDamage),
    // Secondary Elemental Damage
    Combined(CombinedElementalDamage),

    #[serde(deserialize_with = "deserialize_ignore_any")]
    Other,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, derive_more::Display, Hash)]
#[serde(rename_all = "lowercase")]
pub enum PhysicalDamage {
    Impact,
    Puncture,
    Slash,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, derive_more::Display, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ElementalDamage {
    Heat,
    Cold,
    Electricity,
    Toxin,
    Void,
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

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, derive_more::Display, Hash)]
#[serde(rename_all = "lowercase")]
pub enum CombinedElementalDamage {
    Radiation,
    Blast,
    Viral,
    Corrosive,
    Gas,
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
