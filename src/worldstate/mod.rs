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

pub mod language;

/// The prelude which contains most things you need.
pub mod prelude {

    pub use crate::worldstate::language::Language;
    pub use crate::worldstate::{
        client::Client,
        error::{
            ApiErrorResponse,
            Error,
        },
        models::*, // most of `base.rs` is included here
    };
}

/// Represents what has happened to the nested Item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Change {
    /// The Item has been added to the collection
    Added,
    /// The Item has been removed the collection
    Removed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CrossDiff<'a, T>
where
    T: PartialEq,
{
    current: &'a [T],
    incoming: &'a [T],
}

impl<'a, T> CrossDiff<'a, T>
where
    T: PartialEq,
{
    /// Creates a [`CrossDiff`]
    pub fn new(current: &'a [T], incoming: &'a [T]) -> Self {
        Self { current, incoming }
    }

    /// Gets all the removed items
    #[must_use]
    pub fn removed(&self) -> Vec<(&'a T, Change)> {
        self.current
            .iter()
            .filter(|&item| !self.incoming.contains(item))
            .map(|item| (item, Change::Removed))
            .collect()
    }

    /// Gets all the added items
    #[must_use]
    pub fn added(&self) -> Vec<(&'a T, Change)> {
        self.incoming
            .iter()
            .filter(|&item| !self.current.contains(item))
            .map(|item| (item, Change::Added))
            .collect()
    }
}
