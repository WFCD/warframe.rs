use super::error::{ApiError, ApiErrorResponse};

use super::models::base::{Endpoint, Model, RTArray, RTObject};

#[derive(Default, Debug, Clone)]
pub struct Client {
    session: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Default::default()
    }
}

// impl FETCH
impl Client {
    pub async fn fetch<T>(&self) -> Result<T, ApiError>
    where
        T: Model + Endpoint + RTObject,
    {
        let response = self.session.get(T::endpoint_en()).send().await?;

        if response.status().is_success() {
            let json_result = response.json::<T>().await?;
            Ok(json_result)
        } else {
            let error_response = response.json::<ApiErrorResponse>().await?;
            Err(ApiError::ApiError(error_response))
        }
    }

    pub async fn fetch_arr<T>(&self) -> Result<Vec<T>, ApiError>
    where
        T: Model + Endpoint + RTArray,
    {
        let response = self.session.get(T::endpoint_en()).send().await?;

        if response.status().is_success() {
            let json_result = response.json::<Vec<T>>().await?;
            Ok(json_result)
        } else {
            let error_response = response.json::<ApiErrorResponse>().await?;
            Err(ApiError::ApiError(error_response))
        }
    }
}

// impl UPDATE LISTENER
#[cfg(feature = "worldstate_listeners")]
impl Client {
    /// Asynchronous method that continuously fetches updates for a given type `T` and invokes a callback function.
    ///
    /// # Arguments
    ///
    /// - `callback`: A function that implements the `ListenerCallback` trait and is called with the previous and new values of `T`.
    ///
    /// # Generic Constraints
    ///
    /// - `T`: Must implement the `Model`, `Endpoint`, `RTObject`, and `TimedEvent` traits.
    /// - `Callback`: Must implement the `ListenerCallback` trait with a lifetime parameter `'any` and type parameter `T`.
    ///
    /// # Returns
    ///
    /// - `Result<(), ApiError>`: Returns `Ok(())` if the operation is successful, otherwise returns an `ApiError`.
    ///
    /// # Example
    ///
    /// ```rust
    ///use std::error::Error;
    ///
    ///use warframe::worldstate::prelude::*;
    ///
    ///async fn on_cetus_update(before: &Cetus, after: &Cetus) {
    ///    println!("BEFORE : {before:?}");
    ///    println!("AFTER  : {after:?}");
    ///}
    ///
    ///#[tokio::main]
    ///async fn main() -> Result<(), Box<dyn Error>> {
    ///    env_logger::builder()
    ///        .filter_level(log::LevelFilter::Debug)
    ///        .init();
    ///
    ///    let client = Client::new();
    ///    
    ///    client.call_on_update(on_cetus_update); // don't forget to start it as a bg task (or .await it)s
    ///    Ok(())
    ///}
    ///
    /// ```
    pub async fn call_on_update<T, Callback>(&self, callback: Callback) -> Result<(), ApiError>
    where
        T: Model + Endpoint + RTObject + crate::worldstate::models::TimedEvent,
        for<'any> Callback: crate::worldstate::listener::ListenerCallback<'any, T>,
    {
        log::debug!("{} (LISTENER) :: Started", std::any::type_name::<T>());
        let mut item = self.fetch::<T>().await?;

        loop {
            if item.expiry() <= chrono::offset::Utc::now() {
                log::debug!(
                    "{} (LISTENER) :: Fetching new possible update",
                    std::any::type_name::<T>()
                );
                tokio::time::sleep(std::time::Duration::from_secs(30)).await;

                let new_item = self.fetch::<T>().await?;

                if item.expiry() >= new_item.expiry() {
                    continue;
                }
                callback.call(&item, &new_item).await;
                item = new_item;
            }

            let time_to_sleep = item.expiry() - chrono::offset::Utc::now();

            log::debug!(
                "{} (LISTENER) :: Sleeping {} seconds",
                std::any::type_name::<T>(),
                time_to_sleep.num_seconds()
            );
            tokio::time::sleep(time_to_sleep.to_std().unwrap()).await;
        }
    }

    /// Asynchronous method that continuously fetches updates for a given type `T` and invokes a callback function.
    ///
    /// # Arguments
    ///
    /// - `callback`: A function that implements the `ListenerCallback` trait and is called with the previous and new values of `T`.
    ///
    /// # Generic Constraints
    ///
    /// - `T`: Must implement the `Model`, `Endpoint`, `RTObject`, and `TimedEvent` traits.
    /// - `Callback`: Must implement the `ListenerCallback` trait with a lifetime parameter `'any` and type parameter `T`.
    ///
    /// # Returns
    ///
    /// - `Result<(), ApiError>`: Returns `Ok(())` if the operation is successful, otherwise returns an `ApiError`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::error::Error;
    ///
    /// use warframe::worldstate::{listener::Change, prelude::*};
    ///
    /// /// This function will be called once a fissure updates.
    /// /// This will send a request to the corresponding endpoint once every 30s
    /// /// and compare the results for changes.
    /// async fn on_fissure_update(fissure: Change<'_, Fissure>) {
    ///     match fissure {
    ///         Change::Added(fissure) => println!("Fissure ADDED   : {fissure:?}"),
    ///         Change::Removed(fissure) => println!("Fissure REMOVED : {fissure:?}"),
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     // Logging setup
    ///     env_logger::builder()
    ///         .filter_level(log::LevelFilter::Debug)
    ///         .init();
    ///
    ///     // initialize a client (included in the prelude)
    ///     let client = Client::new();
    ///
    ///     // Pass the function to the handler
    ///     // (will return a Future)
    ///     client.call_on_nested_update(on_fissure_update); // don't forget to start it as a bg task (or .await it)
    ///     Ok(())
    /// }
    ///
    /// ```
    pub async fn call_on_nested_update<T, Callback>(
        &self,
        callback: Callback,
    ) -> Result<(), ApiError>
    where
        T: Model + Endpoint + RTArray + crate::worldstate::models::TimedEvent + PartialEq,
        for<'any> Callback: crate::worldstate::listener::NestedListenerCallback<'any, T>,
    {
        log::debug!("{} (LISTENER) :: Started", std::any::type_name::<Vec<T>>());
        let mut items = self.fetch_arr::<T>().await?;

        loop {
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;

            log::debug!(
                "{} (LISTENER) :: Fetching new possible state",
                std::any::type_name::<Vec<T>>()
            );
            let new_items = self.fetch_arr::<T>().await?;

            let diff = crate::worldstate::listener::CrossDiff::new(&items, &new_items);

            let removed_items = diff.removed();
            let added_items = diff.added();

            if !removed_items.is_empty() || !added_items.is_empty() {
                log::debug!(
                    "{} (LISTENER) :: Found changes, proceeding to call callback with every change",
                    std::any::type_name::<Vec<T>>()
                );
                for item in removed_items.into_iter().chain(added_items) {
                    // call callback fn
                    callback.call(item).await;
                }
                items = new_items;
            }
        }
    }
}

// impl UPDATE LISTENER (with state)
#[cfg(feature = "worldstate_listeners")]
impl Client {
    /// Asynchronous method that calls a callback function with state on update.
    ///
    /// # Arguments
    ///
    /// - `callback`: A callback function that takes the current item, the new item, and the state as arguments.
    /// - `state`: The state object that will be passed to the callback function.
    ///
    /// # Generic Parameters
    ///
    /// - `S`: The type of the state object. It must be `Sized`, `Send`, `Sync`, and `Clone`.
    /// - `T`: The type of the item. It must implement the `Model`, `Endpoint`, `RTObject`, and `TimedEvent` traits.
    /// - `Callback`: The type of the callback function. It must implement the `StatefulListenerCallback` trait with the item type `T` and the state type `S`.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` indicating whether the operation was successful or an `ApiError` occurred.
    /// The result is `Ok(())` if the operation was successful.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::{error::Error, sync::Arc};
    ///
    /// use warframe::worldstate::prelude::*;
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
    ///     env_logger::builder()
    ///         .filter_level(log::LevelFilter::Debug)
    ///         .init();
    ///
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
    pub async fn call_on_update_with_state<S, T, Callback>(
        &self,
        callback: Callback,
        state: S,
    ) -> Result<(), ApiError>
    where
        S: Sized + Send + Sync + Clone,
        T: Model + Endpoint + RTObject + crate::worldstate::models::TimedEvent,
        for<'any> Callback: crate::worldstate::listener::StatefulListenerCallback<'any, T, S>,
    {
        let mut item = self.fetch::<T>().await?;

        loop {
            if item.expiry() <= chrono::offset::Utc::now() {
                log::debug!(
                    "{} (LISTENER) :: Fetching possible update",
                    std::any::type_name::<T>()
                );
                tokio::time::sleep(std::time::Duration::from_secs(30)).await;

                let new_item = self.fetch::<T>().await?;

                if item.expiry() >= new_item.expiry() {
                    continue;
                }
                callback
                    .call_with_state(state.clone(), &item, &new_item)
                    .await;
                item = new_item;
            }

            let time_to_sleep = item.expiry() - chrono::offset::Utc::now();

            log::debug!(
                "{} (LISTENER) :: Sleeping {} seconds",
                std::any::type_name::<T>(),
                time_to_sleep.num_seconds()
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
    /// * `T` - The type of the items, which must implement the `Model`, `Endpoint`, `RTArray`, `TimedEvent`, and `PartialEq` traits.
    /// * `Callback` - The type of the callback function, which must implement the `StatefulNestedListenerCallback` trait.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the callback function is successfully called on each change, or an `ApiError` if an error occurs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::{error::Error, sync::Arc};
    ///
    /// use warframe::worldstate::{listener::Change, prelude::*};
    ///
    /// // Define some state
    /// #[derive(Debug)]
    /// struct MyState {
    ///     _num: i32,
    ///     _s: String,
    /// }
    ///
    /// async fn on_fissure_update(state: Arc<MyState>, fissure: Change<'_, Fissure>) {
    ///     println!("STATE  : {state:?}");
    ///     match fissure {
    ///         Change::Added(f) => println!("FISSURE ADDED   : {f:?}"),
    ///         Change::Removed(f) => println!("FISSURE REMOVED : {f:?}"),
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     env_logger::builder()
    ///         .filter_level(log::LevelFilter::Debug)
    ///         .init();
    ///
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
    pub async fn call_on_nested_update_with_state<S, T, Callback>(
        &self,
        callback: Callback,
        state: S,
    ) -> Result<(), ApiError>
    where
        S: Sized + Send + Sync + Clone,
        T: Model + Endpoint + RTArray + crate::worldstate::models::TimedEvent + PartialEq,
        for<'any> Callback: crate::worldstate::listener::StatefulNestedListenerCallback<'any, T, S>,
    {
        log::debug!("{} (LISTENER) :: Started", std::any::type_name::<Vec<T>>());
        let mut items = self.fetch_arr::<T>().await?;

        loop {
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;

            log::debug!(
                "{} (LISTENER) :: Fetching new possible state",
                std::any::type_name::<Vec<T>>()
            );
            let new_items = self.fetch_arr::<T>().await?;

            let diff = crate::worldstate::listener::CrossDiff::new(&items, &new_items);

            let removed_items = diff.removed();
            let added_items = diff.added();

            if !removed_items.is_empty() && !added_items.is_empty() {
                log::debug!(
                    "{} (LISTENER) :: Found changes, proceeding to call callback with every change",
                    std::any::type_name::<Vec<T>>()
                );
                for item in removed_items.into_iter().chain(added_items) {
                    // call callback fn
                    callback.call_with_state(state.clone(), item).await;
                }
                items = new_items;
            }
        }
    }
}

// impl FETCH (with language)
#[cfg(feature = "multilangual")]
impl Client {
    pub async fn fetch_using_lang<T>(
        &self,
        language: super::language::Language,
    ) -> Result<T, ApiError>
    where
        T: Model + Endpoint + RTObject,
    {
        let response = self
            .session
            .get(T::endpoint(language))
            .send()
            .await
            .unwrap();

        if response.status().is_success() {
            let json_result = response.json::<T>().await?;
            Ok(json_result)
        } else {
            let error_response = response.json::<ApiErrorResponse>().await?;
            Err(ApiError::ApiError(error_response))
        }
    }

    pub async fn fetch_arr_using_lang<T>(
        &self,
        language: super::language::Language,
    ) -> Result<Vec<T>, ApiError>
    where
        T: Model + Endpoint + RTArray,
    {
        let response = self
            .session
            .get(T::endpoint(language))
            .send()
            .await
            .unwrap();

        if response.status().is_success() {
            let json_result = response.json::<Vec<T>>().await?;
            Ok(json_result)
        } else {
            let error_response = response.json::<ApiErrorResponse>().await?;
            Err(ApiError::ApiError(error_response))
        }
    }
}
