pub mod client;
pub mod error;
pub mod models;

#[cfg(feature = "multilangual")]
pub mod language;

#[cfg(feature = "worldstate_listeners")]
pub mod listener;

pub mod prelude {
    pub use crate::worldstate::client::Client;
    pub use crate::worldstate::error::ApiError;

    #[cfg(feature = "multilangual")]
    pub use crate::worldstate::language::Language;

    pub use crate::worldstate::models::*; // most of `base.rs` is included here
}
