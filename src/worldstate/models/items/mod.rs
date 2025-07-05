//! Everything to do with the item type - whether it's Warframes, Weapons, Mods, everything.

pub use arcane::Arcane;
pub use archwing::Archwing;
pub use chrono::NaiveDate;
pub use gear::Gear;
pub use minimal_item::MinimalItem;
pub use misc::Misc;
pub use modification::Mod;
pub use node::Node;
pub use pet::Pet;
pub use relic::Relic;
pub use resource::Resource;
pub use sentinel::Sentinel;
pub use serde::Deserialize;
pub use warframe::{
    Ability,
    Sex,
    Warframe,
};
use weapon::Weapon;

mod arcane;
mod archwing;
mod gear;
mod minimal_item;
mod misc;
mod modification;
mod node;
mod pet;
mod relic;
mod resource;
mod sentinel;
mod warframe;
pub mod weapon;

/// Represents a polarity
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, derive_more::Display)]
#[serde(rename_all = "lowercase")]
pub enum Polarity {
    /// V (Damage, Powers) - Commonly dropped by Grineer
    Madurai,

    /// D (Defensive, Health, Armor) - Dropped by all factions
    Vazarin,

    /// Dash/Bar (Utility, Misc.) - Commonly dropped by Corpus
    Naramon,

    /// Mainly used for Warframe Augment Mods, in addition to some Melee Stance Mods
    Zenurik,

    /// Used for certain Melee Stance Mods
    Unairu,

    /// Y (Companion Abilities) - Dropped by all factions
    Penjaga,

    /// U (Anti-Sentient Mods) - Obtained upon completion of The Sacrifice
    Umbra,

    /// O (Universal polarity) - Can be only applied by Aura- and Stance-Forma on their slots
    /// respectively
    Any,
}

/// A component for building said item
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub unique_name: String,

    pub name: String,

    pub description: String,

    pub item_count: i64,

    pub image_name: String,

    pub tradable: bool,

    pub masterable: bool,

    pub drops: Vec<Drop>,
}

/// An "upgrade step". Most arcanes have 5 levels, here are each level's description.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct LevelStat {
    pub stats: Vec<String>,
}

/// Information about a component's drop
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Drop {
    pub chance: f64,

    pub location: String,

    pub rarity: Rarity,

    #[serde(rename = "type")]
    pub drop_type: String,
}

/// Information about an Item's introduction
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Introduced {
    pub name: String,

    pub url: String,

    pub aliases: Vec<String>,

    pub parent: String,

    pub date: NaiveDate,
}

/// An Item's category
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
    /// Arcanes
    Arcanes,
    /// Archwing
    Archwing,
    /// Fish
    Fish,
    /// Items equippable in the gear wheel
    Gear,
    /// Glyphs
    Glyphs,
    /// Misc
    Misc,
    /// Mods
    Mods,
    /// Node
    Node,
    /// Pets
    Pets,
    /// Quests
    Quests,
    /// Relics
    Relics,
    /// Resources
    Resources,
    /// Sentinels
    Sentinels,
    /// Sigils
    Sigils,
    /// Skins
    Skins,
    /// Warframes
    Warframes,

    // --- Guns ---
    /// Primary Weapons
    Primary,
    /// Secondary Weapons
    Secondary,
    /// `ArchGun` Weapons
    #[serde(rename = "Arch-Gun")]
    ArchGun,

    // --- Melees ---
    /// Melee Weapons
    Melee,
    #[serde(rename = "Arch-Melee")]
    /// `ArchMelee` Weapons
    ArchMelee,
}

/// An Item's rarity
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum Rarity {
    /// Common
    Common,
    /// Uncommon
    Uncommon,
    /// Rare
    Rare,
    /// Legendary
    Legendary,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "category")]
pub enum Item {
    #[serde(rename = "Arcanes")]
    Arcane(Arcane),
    Archwing(Archwing),
    Fish(MinimalItem),
    Gear(Gear),
    #[serde(rename = "Glyphs")]
    Glyph(MinimalItem),
    Misc(Misc),
    #[serde(rename = "Mods")]
    Mod(Mod),
    Node(Node),
    #[serde(rename = "Pets")]
    Pet(Pet),
    #[serde(rename = "Quests")]
    Quest(MinimalItem),
    #[serde(rename = "Relics")]
    Relic(Relic),
    #[serde(rename = "Resources")]
    Resource(Resource),
    #[serde(rename = "Sentinels")]
    Sentinel(Sentinel),
    #[serde(rename = "Sigils")]
    Sigil(MinimalItem),
    #[serde(rename = "Skins")]
    Skin(MinimalItem),
    // boxed because it's fairly large - enums always take as much space as the largest element
    #[serde(rename = "Warframes")]
    Warframe(Box<Warframe>),

    #[serde(
        alias = "Primary",
        alias = "Secondary",
        alias = "Melee",
        alias = "Arch-Gun",
        alias = "Arch-Melee"
    )]
    Weapon(Box<Weapon>),
}
#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::worldstate::{
        fixtures::item::{
            item_sigil_en,
            nanospores_de,
        },
        models::items::Item,
    };

    #[rstest]
    fn test_item_query(item_sigil_en: &str, nanospores_de: &str) {
        let sigil = serde_json::from_str::<Item>(item_sigil_en).unwrap();

        assert!(matches!(sigil, Item::Sigil(_)));

        let nanospores = serde_json::from_str::<Item>(nanospores_de).unwrap();

        assert!(matches!(nanospores, Item::Misc(_)));
    }
}
