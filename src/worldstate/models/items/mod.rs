//! Everything to do with the item type - whether it's Warframes, Weapons, Mods, everything.

use serde::{Deserialize, Serialize};

pub mod warframe;

/// Represents a polarity
#[derive(Debug, Deserialize, Serialize, strum::Display)]
pub enum Polarity {
    /// V (Damage, Powers) - Commonly dropped by Grineer
    #[serde(rename = "madurai")]
    Madurai,

    /// D (Defensive, Health, Armor) - Dropped by all factions
    #[serde(rename = "vazarin")]
    Vazarin,

    /// Dash/Bar (Utility, Misc.) - Commonly dropped by Corpus
    #[serde(rename = "naramon")]
    Naramon,

    /// Mainly used for Warframe Augment Mods, in addition to some Melee Stance Mods
    #[serde(rename = "zenurik")]
    Zenurik,

    /// Used for certain Melee Stance Mods
    #[serde(rename = "unairu")]
    Unairu,

    /// Y (Companion Abilities) - Dropped by all factions
    #[serde(rename = "penjaga")]
    Penjaga,

    /// U (Anti-Sentient Mods) - Obtained upon completion of The Sacrifice
    #[serde(rename = "umbra")]
    Umbra,

    /// O (Universal polarity) - Can be only applied by Aura- and Stance-Forma on their slots respectively
    #[serde(rename = "any")]
    Any,
}

/// A component for building said item
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    unique_name: String,

    name: String,

    description: String,

    item_count: i64,

    image_name: String,

    tradable: bool,

    masterable: bool,

    drops: Vec<Drop>,
}

/// Information about a component's drop
#[derive(Serialize, Deserialize)]
pub struct Drop {
    chance: f64,

    location: String,

    rarity: String,

    #[serde(rename = "type")]
    drop_type: String,
}

/// Information about an Item's introduction
#[derive(Serialize, Deserialize)]
pub struct Introduced {
    name: String,

    url: String,

    aliases: Vec<String>,

    parent: String,

    date: String,
}



#[test]
fn test_to_string() {
    assert_eq!(Polarity::Madurai.to_string(), "Madurai");
}
