use crate::worldstate::{
    Change,
    Client,
    Error,
    Queryable,
    TimedEvent,
    utils::CrossDiff,
};

fn ignore_state<F, T>(f: F) -> impl for<'a> AsyncFn((), &'a T, Change)
where
    F: AsyncFn(&T, Change),
{
    async move |(), item, change| f(item, change).await
}

impl Client {
    /// Asynchronous method that calls a callback function on nested updates with a given state.
    /// Used on types that yield many data at once - such as fissures.
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
    ///     let client = Client::default();
    ///
    ///     // Pass the function to the handler
    ///     // (will return a Future)
    ///     client.call_on_nested_update(on_fissure_update); // don't forget to start it as a bg task (or .await it)
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
    pub async fn call_on_nested_update<T, Callback>(&self, callback: Callback) -> Result<(), Error>
    where
        T: TimedEvent + Queryable<Return = Vec<T>> + PartialEq,
        for<'any> Callback: AsyncFn(&'any T, Change),
    {
        self.call_on_nested_update_inner(ignore_state(callback), ())
            .await
    }

    /// Same as [`Client::call_on_nested_update`], but with an additional provided state.
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
    ///     let client = Client::default();
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
    #[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
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
        self.call_on_nested_update_inner(callback, state).await
    }

    /// A generalized implementation of stateful and non-stateful nested listeners.
    async fn call_on_nested_update_inner<S, T, Callback>(
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
            tokio::time::sleep(self.config.nested_listener_sleep).await;

            tracing::debug!(
                listener = %std::any::type_name::<Vec<T>>(),
                "(LISTENER) Fetching new possible state"
            );

            let new_items = self.fetch::<T>().await?;

            let diff = CrossDiff::new(&items, &new_items);

            let removed_items = diff.removed();
            let added_items = diff.added();

            if removed_items.is_empty() && added_items.is_empty() {
                continue;
            }

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
