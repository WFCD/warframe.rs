#[cfg(feature = "worldstate")]
pub mod worldstate;

pub(crate) mod ws {
    pub(crate) use crate::worldstate::models::base::*;
    pub(crate) use crate::worldstate::models::macros::{
        impl_endpoint, impl_model_struct, impl_timed_event,
    };
}
