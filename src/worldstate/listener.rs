//! # Allows registering listeners.
//!
//! Listeners are functions that will be called when a specific endpoint receives an update.
//!
//! There are 2 types. The 'normal' ones for models like [Cetus](crate::worldstate::models::Cetus),
//! and 'nested' ones for models like [Fissure](crate::worldstate::models::Fissure).
//!
//! ### Demo for the 'normal' listeners
//! ```rust,no_run
//! use std::error::Error;
//!
//! use warframe::worldstate::prelude::*;
//!
//! async fn on_cetus_update(before: &Cetus, after: &Cetus) {
//!     println!("BEFORE : {before:?}");
//!     println!("AFTER  : {after:?}");
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     env_logger::builder()
//!         .filter_level(log::LevelFilter::Debug)
//!         .init();
//!
//!     let client = Client::new();
//!     
//!     client.call_on_update(on_cetus_update); // don't forget to start it as a bg task (or .await it)
//!     Ok(())
//! }
//! ```
//!
//! ### Demo for the 'nested' listeners
//! ```rust
//! use std::error::Error;
//!
//! use warframe::worldstate::{listener::Change, prelude::*};
//!
//! /// This function will be called once a fissure updates.
//! /// This will send a request to the corresponding endpoint once every 30s
//! /// and compare the results for changes.
//! async fn on_fissure_update(fissure: &Fissure, change: Change) {
//!     match change {
//!         Change::Added => println!("Fissure ADDED   : {fissure:?}"),
//!         Change::Removed => println!("Fissure REMOVED : {fissure:?}"),
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     // Logging setup
//!     env_logger::builder()
//!         .filter_level(log::LevelFilter::Debug)
//!         .init();
//!
//!     // initialize a client (included in the prelude)
//!     let client = Client::new();
//!
//!     // Pass the function to the handler
//!     // (will return a Future)
//!     client.call_on_nested_update(on_fissure_update); // don't forget to start it as a bg task (or .await it)
//!     Ok(())
//! }
//! ```

use std::future::Future;

/// Represents what has happened to the nested Item.
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Change {
    /// The Item has been added to the collection
    Added,
    /// The Item has been removed the collection
    Removed,
}

// ----------
/// A listener callback that can listen to any model with
/// [`Queryable::Return`](crate::worldstate::models::base::Queryable::Return) = Self
pub trait ListenerCallback<'a, T>
where
    T: Sized + 'a,
{
    /// Type of the future
    type Fut: Future + Send;
    /// The method to call the handler
    fn call(&self, before: &'a T, after: &'a T) -> Self::Fut;
}

impl<'a, T, Fut, Func> ListenerCallback<'a, T> for Func
where
    T: Sized + 'a,
    Fut: Future + Send,
    Func: Fn(&'a T, &'a T) -> Fut,
{
    type Fut = Fut;
    fn call(&self, before: &'a T, after: &'a T) -> Self::Fut {
        self(before, after)
    }
}

/// A listener callback that can listen to any model with
/// [`Queryable::Return`](crate::worldstate::models::base::Queryable::Return) = Vec<Self>
pub trait NestedListenerCallback<'a, T>
where
    T: Sized,
{
    /// Type of the future
    type Fut: Future + Send;
    /// The method to call the handler
    fn call(&self, item: &'a T, change: Change) -> Self::Fut;
}

impl<'a, T, Fut, Func> NestedListenerCallback<'a, T> for Func
where
    T: Sized + 'a,
    Fut: Future + Send,
    Func: Fn(&'a T, Change) -> Fut,
{
    type Fut = Fut;
    fn call(&self, item: &'a T, change: Change) -> Self::Fut {
        self(item, change)
    }
}

// --------- STATEFUL CALLBACKS
/// A listener callback that can listen to any model with
/// [`Queryable::Return`](crate::worldstate::models::base::Queryable::Return) = Self
///
/// and a state.
pub trait StatefulListenerCallback<'a, T, S>
where
    T: Sized,
    S: Sized + Send + Sync,
{
    /// Type of the future
    type Fut: Future + Send;
    /// The method to call the handler
    fn call_with_state(&self, state: S, before: &'a T, after: &'a T) -> Self::Fut;
}

impl<'a, T, Fut, Func, S> StatefulListenerCallback<'a, T, S> for Func
where
    T: Sized + 'a,
    S: Sized + Send + Sync,
    Fut: Future + Send,
    Func: Fn(S, &'a T, &'a T) -> Fut,
{
    type Fut = Fut;
    fn call_with_state(&self, state: S, before: &'a T, after: &'a T) -> Self::Fut {
        self(state, before, after)
    }
}

/// A listener callback that can listen to any model with
/// [`Queryable::Return`](crate::worldstate::models::base::Queryable::Return) = Vec<Self>
///
/// and a state.
pub trait StatefulNestedListenerCallback<'a, T, S>
where
    T: Sized,
    S: Sized + Send + Sync,
{
    /// Type of the future
    type Fut: Future + Send;
    /// The method to call the handler
    fn call_with_state(&self, state: S, item: &'a T, change: Change) -> Self::Fut;
}

impl<'a, T, Fut, Func, S> StatefulNestedListenerCallback<'a, T, S> for Func
where
    T: Sized + 'a,
    S: Sized + Send + Sync,
    Fut: Future + Send,
    Func: Fn(S, &'a T, Change) -> Fut,
{
    type Fut = Fut;
    fn call_with_state(&self, state: S, item: &'a T, change: Change) -> Self::Fut {
        self(state, item, change)
    }
}

/// A type that implements logic to find which items have been added or removed in 2 collections
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
    /// Creates a [CrossDiff]
    pub fn new(current: &'a [T], incoming: &'a [T]) -> Self {
        Self { current, incoming }
    }

    /// Gets all the removed items
    pub fn removed(&self) -> Vec<(&'a T, Change)> {
        self.current
            .iter()
            .filter(|&item| !self.incoming.contains(item))
            .map(|item| (item, Change::Removed))
            .collect()
    }

    /// Gets all the added items
    pub fn added(&self) -> Vec<(&'a T, Change)> {
        self.incoming
            .iter()
            .filter(|&item| !self.current.contains(item))
            .map(|item| (item, Change::Added))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use std::{
        sync::Arc,
        vec,
    };

    use super::{
        Change,
        CrossDiff,
    };
    use crate::worldstate::prelude::{
        Cetus,
        Fissure,
    };

    async fn on_cetus_update(_before: &Cetus, _after: &Cetus) {}
    async fn on_cetus_update_stateful(_state: Arc<i32>, _before: &Cetus, _after: &Cetus) {}
    async fn on_fissure_update_nested(_item: &Fissure, _change: Change) {}
    async fn on_fissure_update_stateful_nested(_state: Arc<i32>, _item: &Fissure, _change: Change) {
    }

    #[tokio::test]
    async fn test() {
        use crate::worldstate::prelude::*;
        let client = Arc::new(Client::new());

        let cloned = client.clone();
        tokio::task::spawn(async move { cloned.call_on_update(on_cetus_update).await });
        let cloned = client.clone();
        tokio::task::spawn(async move {
            cloned
                .call_on_update_with_state(on_cetus_update_stateful, Arc::new(4))
                .await
        });
        let cloned = client.clone();
        tokio::task::spawn(
            async move { cloned.call_on_nested_update(on_fissure_update_nested).await },
        );
        let cloned = client.clone();
        tokio::task::spawn(async move {
            cloned
                .call_on_nested_update_with_state(on_fissure_update_stateful_nested, Arc::new(4))
                .await
        });
    }

    #[test]
    fn test_crossdiff() {
        let a = vec![1, 2, 3];
        let b = vec![1, 3, 4];
        let cf = CrossDiff::new(&a, &b);
        assert_eq!(cf.added(), vec![(&4, Change::Added)]);
        assert_eq!(cf.removed(), vec![(&2, Change::Removed)]);
    }
}
