#![allow(clippy::missing_errors_doc)]

//! A client to do all sorts of things with the API

use reqwest::StatusCode;

use super::{
    Queryable,
    TimedEvent,
    error::Error,
    items::{
        Category,
        map_category_to_item,
    },
    language::Language,
    models::items::Item,
    utils::{
        Change,
        CrossDiff,
    },
};

/// The client that acts as a convenient way to query models.
///
/// ## Example
/// ```rust
/// use warframe::worldstate::{
///     Client,
///     Error,
///     queryable::{
///         Cetus,
///         Fissure,
///     },
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     let client = Client::new();
///
///     let cetus: Cetus = client.fetch::<Cetus>().await?;
///     let fissures: Vec<Fissure> = client.fetch::<Fissure>().await?;
///
///     Ok(())
/// }
/// ```
///
/// Check the [queryable](crate::worldstate::queryable) module for all queryable types.
#[derive(Default, Debug, Clone)]
pub struct Client {
    session: reqwest::Client,
}

impl Client {
    /// Creates a new [Client].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

// impl FETCH
impl Client {
    /// Fetches an instance of a specified model.
    ///
    /// # Example
    /// ```rust,no_run
    /// use warframe::worldstate::{
    ///     Client,
    ///     Error,
    ///     queryable::{
    ///         Cetus,
    ///         Fissure,
    ///     },
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///
    ///     let cetus: Cetus = client.fetch::<Cetus>().await?;
    ///     let fissures: Vec<Fissure> = client.fetch::<Fissure>().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn fetch<T>(&self) -> Result<T::Return, Error>
    where
        T: Queryable,
    {
        <T as Queryable>::query(&self.session).await
    }

    /// Fetches an instance of a specified model in a supplied Language.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use warframe::worldstate::{
    ///     Client,
    ///     Error,
    ///     Language,
    ///     queryable::{
    ///         Cetus,
    ///         Fissure,
    ///     },
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///
    ///     let cetus: Cetus = client.fetch_using_lang::<Cetus>(Language::ZH).await?;
    ///     let fissures: Vec<Fissure> = client.fetch_using_lang::<Fissure>(Language::ZH).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn fetch_using_lang<T>(&self, language: Language) -> Result<T::Return, Error>
    where
        T: Queryable,
    {
        T::query_with_language(&self.session, language).await
    }

    /// Queries an item by its name and returns the closest matching item.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use warframe::worldstate::{
    ///     Client,
    ///     Error,
    ///     items::Item,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///
    ///     let sigil = client.query_item("Accord Sigil").await?;
    ///
    ///     assert!(matches!(sigil, Some(Item::Sigil(_))));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn query_item(&self, query: &str) -> Result<Option<Item>, Error> {
        self.query_by_url(format!(
            "https://api.warframestat.us/items/{}/?language=en",
            urlencoding::encode(query),
        ))
        .await
    }

    /// Queries an item by its name and returns the closest matching item.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use warframe::worldstate::{
    ///     Client,
    ///     Error,
    ///     Language,
    ///     items::Item,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///
    ///     let nano_spores = client
    ///         .query_item_using_lang("Nanosporen", Language::DE)
    ///         .await?;
    ///
    ///     assert!(matches!(nano_spores, Some(Item::Misc(_))));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn query_item_using_lang(
        &self,
        query: &str,
        language: Language,
    ) -> Result<Option<Item>, Error> {
        self.query_by_url(format!(
            "https://api.warframestat.us/items/{}/?language={}",
            urlencoding::encode(query),
            language
        ))
        .await
    }

    async fn query_by_url(&self, url: String) -> Result<Option<Item>, Error> {
        #[derive(serde::Deserialize)]
        struct DummyCategory {
            category: Category,
        }

        let response = self.session.get(url).send().await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let json = response.text().await?;

        let category = serde_json::from_str::<DummyCategory>(&json)?.category;

        let item = map_category_to_item(category, &json)?;

        Ok(Some(item))
    }

    /// Asynchronous method that continuously fetches updates for a given type `T` and invokes a
    /// callback function.
    ///
    /// # Arguments
    ///
    /// - `callback`: A function that implements the `ListenerCallback` trait and is called with the
    ///   previous and new values of `T`.
    ///
    /// # Generic Constraints
    ///
    /// - `T`: Must implement the `Queryable` and `TimedEvent` traits.
    /// - `Callback`: Must implement the `ListenerCallback` trait with a lifetime parameter `'any`
    ///   and type parameter `T`.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error>`: Returns `Ok(())` if the operation is successful, otherwise returns an
    ///   `Error`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::error::Error;
    ///
    /// use warframe::worldstate::{
    ///     Client,
    ///     queryable::Cetus,
    /// };
    ///
    /// async fn on_cetus_update(before: &Cetus, after: &Cetus) {
    ///     println!("BEFORE : {before:?}");
    ///     println!("AFTER  : {after:?}");
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let client = Client::new();
    ///     
    ///     client.call_on_update(on_cetus_update); // don't forget to start it as a bg task (or .await it)s
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::missing_panics_doc)]
    pub async fn call_on_update<T, Callback>(&self, callback: Callback) -> Result<(), Error>
    where
        T: TimedEvent + Queryable<Return = T>,
        for<'any> Callback: AsyncFn(&'any T, &'any T),
    {
        tracing::debug!("{} (LISTENER) :: Started", std::any::type_name::<T>());
        let mut item = self.fetch::<T>().await?;

        loop {
            if item.expiry() <= chrono::offset::Utc::now() {
                tracing::debug!(
                    listener = %std::any::type_name::<T>(),
                    "(LISTENER) Fetching new possible update"
                );

                tokio::time::sleep(std::time::Duration::from_secs(30)).await;

                let new_item = self.fetch::<T>().await?;

                if item.expiry() >= new_item.expiry() {
                    continue;
                }

                callback(&item, &new_item).await;
                item = new_item;
            }

            let time_to_sleep = item.expiry() - chrono::offset::Utc::now();

            tracing::debug!(
                listener = %std::any::type_name::<T>(),
                sleep_duration = %time_to_sleep.num_seconds(),
                "(LISTENER) Sleeping"
            );

            tokio::time::sleep(time_to_sleep.to_std().unwrap()).await;
        }
    }

    /// Asynchronous method that continuously fetches updates for a given type `T` and invokes a
    /// callback function.
    ///
    /// # Arguments
    ///
    /// - `callback`: A function that implements the `ListenerCallback` trait and is called with the
    ///   previous and new values of `T`.
    ///
    /// # Generic Constraints
    ///
    /// - `T`: Must implement the `Queryable`, `TimedEvent` and `PartialEq` traits.
    /// - `Callback`: Must implement the `ListenerCallback` trait with a lifetime parameter `'any`
    ///   and type parameter `T`.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error>`: Returns `Ok(())` if the operation is successful, otherwise returns an
    ///   `Error`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::error::Error;
    ///
    /// use warframe::worldstate::{
    ///     Client,
    ///     Change,
    ///     queryable::Fissure,
    /// };
    ///
    /// /// This function will be called once a fissure updates.
    /// /// This will send a request to the corresponding endpoint once every 30s
    /// /// and compare the results for changes.
    /// async fn on_fissure_update(fissure: &Fissure, change: Change) {
    ///     match change {
    ///         Change::Added => println!("Fissure ADDED   : {fissure:?}"),
    ///         Change::Removed => println!("Fissure REMOVED : {fissure:?}"),
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     // initialize a client (included in the prelude)
    ///     let client = Client::new();
    ///
    ///     // Pass the function to the handler
    ///     // (will return a Future)
    ///     client.call_on_nested_update(on_fissure_update); // don't forget to start it as a bg task (or .await it)
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::missing_panics_doc)]
    #[allow(clippy::missing_errors_doc)]
    pub async fn call_on_nested_update<T, Callback>(&self, callback: Callback) -> Result<(), Error>
    where
        T: TimedEvent + Queryable<Return = Vec<T>> + PartialEq,
        for<'any> Callback: AsyncFn(&'any T, Change),
    {
        tracing::debug!(
            listener = %std::any::type_name::<Vec<T>>(),
            "(LISTENER) Started"
        );

        let mut items = self.fetch::<T>().await?;

        loop {
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;

            tracing::debug!(
                listener = %std::any::type_name::<Vec<T>>(),
                "(LISTENER) Fetching new possible state"
            );

            let new_items = self.fetch::<T>().await?;

            let diff = CrossDiff::new(&items, &new_items);

            let removed_items = diff.removed();
            let added_items = diff.added();

            if !removed_items.is_empty() || !added_items.is_empty() {
                tracing::debug!(
                    listener = %std::any::type_name::<Vec<T>>(),
                    "(LISTENER) Found changes, proceeding to call callback with every change"
                );

                for (item, change) in removed_items.into_iter().chain(added_items) {
                    // call callback fn
                    callback(item, change).await;
                }
                items = new_items;
            }
        }
    }

    /// Asynchronous method that calls a callback function with state on update.
    ///
    /// # Arguments
    ///
    /// - `callback`: A callback function that takes the current item, the new item, and the state
    ///   as arguments.
    /// - `state`: The state object that will be passed to the callback function.
    ///
    /// # Generic Parameters
    ///
    /// - `S`: The type of the state object. It must be `Sized`, `Send`, `Sync`, and `Clone`.
    /// - `T`: Must implement the `Queryable` and `TimedEvent` traits.
    /// - `Callback`: The type of the callback function. It must implement the
    ///   `StatefulListenerCallback` trait with the item type `T` and the state type `S`.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` indicating whether the operation was successful or an
    /// `Error` occurred. The result is `Ok(())` if the operation was successful.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::{error::Error, sync::Arc};
    ///
    /// use warframe::worldstate::{Client, queryable::Cetus};
    ///
    /// // Define some state
    /// #[derive(Debug)]
    /// struct MyState {
    ///     _num: i32,
    ///     _s: String,
    /// }
    ///
    /// async fn on_cetus_update(state: Arc<MyState>, before: &Cetus, after: &Cetus) {
    ///     println!("STATE  : {state:?}");
    ///     println!("BEFORE : {before:?}");
    ///     println!("AFTER  : {after:?}");
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let client = Client::new();
    ///
    ///     // Note that the state will be cloned into the handler, so Arc is preferred
    ///     let state = Arc::new(MyState {
    ///         _num: 69,
    ///         _s: "My ginormous ass".into(),
    ///     });
    ///
    ///     client
    ///         .call_on_update_with_state(on_cetus_update, state); // don't forget to start it as a bg task (or .await it)
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::missing_panics_doc)]
    pub async fn call_on_update_with_state<S, T, Callback>(
        &self,
        callback: Callback,
        state: S,
    ) -> Result<(), Error>
    where
        S: Sized + Send + Sync + Clone,
        T: TimedEvent + Queryable<Return = T>,
        for<'any> Callback: AsyncFn(S, &'any T, &'any T),
    {
        let mut item = self.fetch::<T>().await?;

        loop {
            if item.expiry() <= chrono::offset::Utc::now() {
                tracing::debug!(
                    listener = %std::any::type_name::<T>(),
                    "(LISTENER) Fetching new possible state"
                );

                tokio::time::sleep(std::time::Duration::from_secs(30)).await;

                let new_item = self.fetch::<T>().await?;

                if item.expiry() >= new_item.expiry() {
                    continue;
                }
                callback(state.clone(), &item, &new_item).await;
                item = new_item;
            }

            let time_to_sleep = item.expiry() - chrono::offset::Utc::now();

            tracing::debug!(
                listener = %std::any::type_name::<T>(),
                sleep_duration = %time_to_sleep.num_seconds(),
                "(LISTENER) Sleeping"
            );

            tokio::time::sleep(time_to_sleep.to_std().unwrap()).await;
        }
    }

    /// Asynchronous method that calls a callback function on nested updates with a given state.
    ///
    /// # Arguments
    ///
    /// * `callback` - The callback function to be called on each change.
    /// * `state` - The state to be passed to the callback function.
    ///
    /// # Generic Constraints
    ///
    /// * `S` - The type of the state, which must be `Sized`, `Send`, `Sync`, and `Clone`.
    /// * `T` - Must implement the `Queryable`, `TimedEvent` and `PartialEq` traits.
    /// * `Callback` - The type of the callback function, which must implement the
    ///   `StatefulNestedListenerCallback` trait.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the callback function is successfully called on each change, or an
    /// `Error` if an error occurs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::{error::Error, sync::Arc};
    ///
    /// use warframe::worldstate::{Change, Client, queryable::Fissure};
    ///
    /// // Define some state
    /// #[derive(Debug)]
    /// struct MyState {
    ///     _num: i32,
    ///     _s: String,
    /// }
    ///
    /// async fn on_fissure_update(state: Arc<MyState>, fissure: &Fissure, change: Change) {
    ///     println!("STATE  : {state:?}");
    ///     match change {
    ///         Change::Added => println!("FISSURE ADDED   : {fissure:?}"),
    ///         Change::Removed => println!("FISSURE REMOVED : {fissure:?}"),
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let client = Client::new();
    ///
    ///     // Note that the state will be cloned into the handler, so Arc is preferred
    ///     let state = Arc::new(MyState {
    ///         _num: 69,
    ///         _s: "My ginormous ass".into(),
    ///     });
    ///
    ///     client
    ///         .call_on_nested_update_with_state(on_fissure_update, state); // don't forget to start it as a bg task (or .await it)
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::missing_panics_doc)]
    pub async fn call_on_nested_update_with_state<S, T, Callback>(
        &self,
        callback: Callback,
        state: S,
    ) -> Result<(), Error>
    where
        S: Sized + Send + Sync + Clone,
        T: Queryable<Return = Vec<T>> + TimedEvent + PartialEq,
        for<'any> Callback: AsyncFn(S, &'any T, Change),
    {
        tracing::debug!(
            listener = %std::any::type_name::<Vec<T>>(),
            "(LISTENER) Started"
        );

        let mut items = self.fetch::<T>().await?;

        loop {
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;

            tracing::debug!(
                listener = %std::any::type_name::<Vec<T>>(),
                "(LISTENER) Fetching new possible state"
            );

            let new_items = self.fetch::<T>().await?;

            let diff = CrossDiff::new(&items, &new_items);

            let removed_items = diff.removed();
            let added_items = diff.added();

            if !removed_items.is_empty() || !added_items.is_empty() {
                tracing::debug!(
                    listener = %std::any::type_name::<Vec<T>>(),
                    "(LISTENER) Found changes, proceeding to call callback with every change"
                );

                for (item, change) in removed_items.into_iter().chain(added_items) {
                    // call callback fn
                    callback(state.clone(), item, change).await;
                }
                items = new_items;
            }
        }
    }
}
