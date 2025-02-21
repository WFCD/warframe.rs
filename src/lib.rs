#![doc = include_str!("../README.md")]

pub mod worldstate;

pub mod market;

pub(crate) mod ws {

    pub(crate) use crate::worldstate::{
        language::Language,
        models::{
            base::*,
            macros::{
                impl_model_struct,
                impl_queryable,
                impl_timed_event,
            },
        },
    };
}
