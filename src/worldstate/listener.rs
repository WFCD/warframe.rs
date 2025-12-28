use crate::worldstate::{
    Client,
    Error,
    Queryable,
    TimedEvent,
};

fn ignore_state<F, T>(f: F) -> impl for<'a, 'b> AsyncFn((), &'a T, &'b T)
where
    F: AsyncFn(&T, &T),
{
    async move |(), before, after| f(before, after).await
}

impl Client {
    /// Asynchronous method that continuously fetches updates for a given type `T` and invokes a
    /// callback function.
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
    ///     let client = Client::default();
    ///     
    ///     client.call_on_update(on_cetus_update); // don't forget to start it as a bg task (or .await it)s
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
    pub async fn call_on_update<T, Callback>(&self, callback: Callback) -> Result<(), Error>
    where
        T: TimedEvent + Queryable<Return = T>,
        for<'a, 'b> Callback: AsyncFn(&'a T, &'b T),
    {
        self.call_on_update_inner(ignore_state(callback), ()).await
    }

    /// Asynchronous method that calls a callback function with state on update.
    ///
    /// # Example
    ///
    /// ```
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
    ///     let client = Client::default();
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
    #[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
    pub async fn call_on_update_with_state<S, T, Callback>(
        &self,
        callback: Callback,
        state: S,
    ) -> Result<(), Error>
    where
        S: Sized + Send + Sync + Clone,
        T: TimedEvent + Queryable<Return = T>,
        for<'a, 'b> Callback: AsyncFn(S, &'a T, &'b T),
    {
        self.call_on_update_inner(callback, state).await
    }

    /// A generalized implementation of stateful and non-stateful listeners.
    ///
    /// # Panics
    /// [`chrono::TimeDelta::to_std`]
    async fn call_on_update_inner<S, T, Callback>(
        &self,
        callback: Callback,
        state: S,
    ) -> Result<(), Error>
    where
        S: Sized + Send + Sync + Clone,
        T: TimedEvent + Queryable<Return = T>,
        for<'a, 'b> Callback: AsyncFn(S, &'a T, &'b T),
    {
        let mut item = self.fetch::<T>().await?;

        loop {
            if item.expiry() <= chrono::offset::Utc::now() {
                tracing::debug!(
                    listener = %std::any::type_name::<T>(),
                    "(LISTENER) Fetching new possible state"
                );

                tokio::time::sleep(self.config.listener_sleep_timeout).await;

                let new_item = self.fetch::<T>().await?;

                if item.expiry() >= new_item.expiry() {
                    continue;
                }
                callback(state.clone(), &item, &new_item).await;
                item = new_item;
            }

            let time_to_sleep = item.expiry() - chrono::Utc::now();

            tracing::debug!(
                listener = %std::any::type_name::<T>(),
                sleep_duration = %time_to_sleep.num_seconds(),
                "(LISTENER) Sleeping"
            );

            tokio::time::sleep(time_to_sleep.to_std().unwrap()).await;
        }
    }
}
