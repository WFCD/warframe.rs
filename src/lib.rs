#![doc = include_str!("../README.md")]

#[cfg(feature = "worldstate")]
pub mod worldstate;

#[cfg(feature = "market")]
pub mod market;

#[cfg(feature = "profile")]
pub mod profile;

pub(crate) mod ws {
    #[cfg(feature = "multilangual")]
    pub(crate) use crate::worldstate::language::Language;
    pub(crate) use crate::worldstate::models::{
        base::*,
        macros::{
            impl_model_struct,
            impl_queryable,
            impl_timed_event,
        },
    };
}
