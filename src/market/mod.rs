//! Implementation for the market module, used to interact with the warframe.market API
//!
//! This module implements the [market V2 API](https://42bytes.notion.site/WFM-Api-v2-Documentation-5d987e4aa2f74b55a80db1a09932459d).
#[cfg(feature = "market_cache")]
pub(crate) mod cache;
mod client;
pub mod error;
pub(crate) mod models;

#[cfg(feature = "market_cache")]
pub use cache::{
    SlugContext,
    Slugs,
};
pub use client::Client;
use derive_more::{
    AsRef,
    Display,
    Into,
};
pub use error::{
    Error,
    Result,
};
use heck::ToSnakeCase;
pub use models::{
    Queryable,
    ResponseBase,
    activity::{
        Activity,
        ActivityType,
    },
    i18n::{
        I18N,
        Language,
    },
    item::{
        Item,
        ItemI18N,
    },
    item_short::{
        ItemShort,
        ItemShortI18N,
    },
    lich_ephemera::LichEphemeraI18N,
    lich_quirk::LichQuirkI18N,
    lich_weapon::LichWeaponI18N,
    location::LocationI18N,
    mission::MissionI18N,
    npc::NpcI18N,
    order::Order,
    riven::RivenI18N,
    riven_attribute::{
        RivenAttributeI18N,
        Unit,
    },
    riven_group::RivenGroup,
    riven_type::RivenType,
    set_items::SetItems,
    sister_ephemera::SisterEphemeraI18N,
    sister_quirk::SisterQuirkI18N,
    sister_weapon::SisterWeaponI18N,
    top_orders::TopOrders,
    top_orders_query_params::TopOrdersQueryParams,
    user_short::{
        Status,
        UserShort,
    },
    versions::{
        VersionApps,
        VersionCollections,
    },
};

/// Re-export of all the models that are queryable
pub mod queryable {
    pub use super::models::{
        lich_ephemera::LichEphemera,
        lich_quirk::LichQuirk,
        lich_weapon::LichWeapon,
        location::Location,
        mission::Mission,
        npc::Npc,
        order_with_user::OrderWithUser,
        riven::Riven,
        riven_attribute::RivenAttribute,
        sister_ephemera::SisterEphemera,
        sister_quirk::SisterQuirk,
        sister_weapon::SisterWeapon,
        versions::Versions,
    };
}

pub const BASE_URL: &str = "https://api.warframe.market/v2";

/// This is a utility newtype struct to help convert user inputs (such as `Acceltra Prime Set`) into
/// slugs (e.g. `acceltra_prime_set`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Into, AsRef)]
#[as_ref(str, String)]
pub struct Slug(String);

impl Slug {
    #[must_use]
    pub fn new(input: &str) -> Self {
        Self(input.to_snake_case())
    }

    /// Creates a new `Slug` from a raw string without converting it to snake case.
    ///
    /// Useful when you already checked a string, and just want the type.
    #[must_use]
    pub fn new_raw(input: &str) -> Self {
        Self(input.to_string())
    }
}

impl From<String> for Slug {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}

impl From<&str> for Slug {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl PartialEq<String> for Slug {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl PartialEq<str> for Slug {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for Slug {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

#[cfg(test)]
mod test {
    use super::Slug;

    #[test]
    fn test_slug() {
        assert_eq!(Slug::new("Acceltra Prime Set"), "acceltra_prime_set");
        assert_eq!(Slug::new("Mirage Prime Set"), "mirage_prime_set");
        assert_eq!(Slug::new("Riot-848 Barrel"), "riot_848_barrel");
    }
}
