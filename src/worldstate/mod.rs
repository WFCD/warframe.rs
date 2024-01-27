pub mod client;
pub mod error;
pub mod models;

#[cfg(feature = "multilangual")]
pub mod language;

pub mod prelude {
    pub use crate::worldstate::client::Client;
    pub use crate::worldstate::error::ApiError;

    #[cfg(feature = "multilangual")]
    pub use crate::worldstate::language::Language;

    pub use crate::worldstate::models::*;
}
