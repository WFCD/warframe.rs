#[cfg(feature = "worldstate")]
pub mod worldstate;

pub(crate) mod ws {
    pub use crate::worldstate::models::base::*;
    pub use crate::worldstate::models::macros::*;
}
