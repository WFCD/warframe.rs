//! Everything to do with the item type - whether it's Warframes, Weapons, Mods, everything.

pub use arcane::Arcane;
pub use archwing::Archwing;
pub use chrono::NaiveDate;
pub use fish::Fish;
pub use gear::Gear;
pub use glyph::Glyph;
pub use misc::Misc;
pub use modification::Mod;
pub use node::Node;
pub use pet::Pet;
pub use quest::Quest;
pub use relic::Relic;
pub use resource::Resource;
pub use sentinel::Sentinel;
pub use serde::Deserialize;
pub use sigil::Sigil;
pub use skin::Skin;
pub use warframe::{
    Ability,
    Sex,
    Warframe,
};
use weapon::Weapon;

mod arcane;
mod archwing;
mod fish;
mod gear;
mod glyph;
mod misc;
mod modification;
mod node;
mod pet;
mod quest;
mod relic;
mod resource;
mod sentinel;
mod sigil;
mod skin;
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
#[serde(untagged)]
pub enum Item {
    Arcane(Arcane),
    Archwing(Archwing),
    Fish(Fish),
    Gear(Gear),
    Glyph(Glyph),
    Misc(Misc),
    Mod(Mod),
    Node(Node),
    Pet(Pet),
    Quest(Quest),
    Relic(Relic),
    Resource(Resource),
    Sentinel(Sentinel),
    Sigil(Sigil),
    Skin(Skin),
    // boxed because it's fairly large - enums always take as much space as the largest element
    Warframe(Box<Warframe>),
    Weapon(Box<Weapon>),
}

pub(crate) fn map_category_to_item(
    category: Category,
    json: &str,
) -> Result<Item, serde_json::Error> {
    use serde_json::from_str;
    let item = match category {
        Category::Arcanes => Item::Arcane(from_str::<Arcane>(json)?),
        Category::Archwing => Item::Archwing(from_str::<Archwing>(json)?),
        Category::Fish => Item::Fish(from_str::<Fish>(json)?),
        Category::Gear => Item::Gear(from_str::<Gear>(json)?),
        Category::Glyphs => Item::Glyph(from_str::<Glyph>(json)?),
        Category::Misc => Item::Misc(from_str::<Misc>(json)?),
        Category::Mods => Item::Mod(from_str::<Mod>(json)?),
        Category::Node => Item::Node(from_str::<Node>(json)?),
        Category::Pets => Item::Pet(from_str::<Pet>(json)?),
        Category::Quests => Item::Quest(from_str::<Quest>(json)?),
        Category::Relics => Item::Relic(from_str::<Relic>(json)?),
        Category::Resources => Item::Resource(from_str::<Resource>(json)?),
        Category::Sentinels => Item::Sentinel(from_str::<Sentinel>(json)?),
        Category::Sigils => Item::Sigil(from_str::<Sigil>(json)?),
        Category::Skins => Item::Skin(from_str::<Skin>(json)?),
        Category::Warframes => Item::Warframe(Box::new(from_str::<Warframe>(json)?)),
        Category::Primary
        | Category::Secondary
        | Category::ArchGun
        | Category::Melee
        | Category::ArchMelee => Item::Weapon(Box::new(from_str::<Weapon>(json)?)),
    };

    Ok(item)
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::worldstate::{
        error::Error,
        fixtures::item::{
            item_sigil_en,
            nanospores_de,
        },
        models::items::{
            Category,
            Item,
            map_category_to_item,
        },
    };

    #[rstest]
    fn test_item_query(item_sigil_en: &str, nanospores_de: &str) -> Result<(), Error> {
        let sigil = map_category_to_item(Category::Sigils, item_sigil_en)?;

        assert!(matches!(sigil, Item::Sigil(_)));

        let nanospores = map_category_to_item(Category::Misc, nanospores_de)?;

        assert!(matches!(nanospores, Item::Misc(_)));

        Ok(())
    }
}
