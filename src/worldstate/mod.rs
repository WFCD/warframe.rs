//! # The worldstate module
//!
//! Get information about various different parts of the game.
//! Check []
//!
//! ## Quickstart
//! ```rust,no_run
//! use warframe::worldstate::prelude as wf;
//! #[tokio::main]
//! async fn main() -> Result<(), wf::ApiError> {
//!     let client = wf::Client::new();
//!
//!     let cetus: wf::Cetus = client.fetch::<wf::Cetus>().await?;
//!     let fissures: Vec<wf::Fissure> = client.fetch::<wf::Fissure>().await?;
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod models;

#[cfg(feature = "multilangual")]
pub mod language;

#[cfg(feature = "worldstate_listeners")]
pub mod listener;

/// The prelude which contains most things you need.
pub mod prelude {
    #[cfg(feature = "multilangual")]
    pub use crate::worldstate::language::Language;
    pub use crate::worldstate::{
        client::Client,
        error::{
            ApiError,
            ApiErrorResponse,
        },
        models::*,
    }; // most of `base.rs` is included here
}
