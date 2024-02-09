#[cfg(feature = "worldstate")]
pub mod worldstate;

pub use chrono;
pub use log;
pub use tokio;

pub(crate) mod ws {
    #[cfg(feature = "multilangual")]
    pub(crate) use crate::worldstate::language::Language;
    pub(crate) use crate::worldstate::models::base::*;
    pub(crate) use crate::worldstate::models::macros::{
        impl_endpoint, impl_model_struct, impl_rt, impl_timed_event,
    };
}
