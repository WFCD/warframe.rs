use std::future::Future;

use super::models::{Endpoint, Model, RTArray, RTObject};

pub enum Change<'a, T> {
    Added(&'a T),
    Removed(&'a T),
}

// ----------
pub trait ListenerCallback<'a, T>
where
    T: Sized + Endpoint + RTObject + Model,
{
    type Fut: Future + Send;
    fn call(&self, before: &'a T, after: &'a T) -> Self::Fut;
}

impl<'a, T, Fut, Func> ListenerCallback<'a, T> for Func
where
    T: Sized + Endpoint + RTObject + Model + 'a,
    Fut: Future + Send,
    Func: Fn(&'a T, &'a T) -> Fut,
{
    type Fut = Fut;
    fn call(&self, before: &'a T, after: &'a T) -> Self::Fut {
        self(before, after)
    }
}

pub trait NestedListenerCallback<'a, T>
where
    T: Sized + Endpoint + RTArray + Model,
{
    type Fut: Future + Send;
    fn call(&self, change: Change<'a, T>) -> Self::Fut;
}

impl<'a, T, Fut, Func> NestedListenerCallback<'a, T> for Func
where
    T: Sized + Endpoint + RTArray + Model + 'a,
    Fut: Future + Send,
    Func: Fn(Change<'a, T>) -> Fut,
{
    type Fut = Fut;
    fn call(&self, change: Change<'a, T>) -> Self::Fut {
        self(change)
    }
}

// --------- STATEFUL CALLBACKS
pub trait StatefulListenerCallback<'a, T, S>
where
    T: Sized + Endpoint + RTObject + Model,
    S: Sized + Send + Sync,
{
    type Fut: Future + Send;
    fn call_with_state(&self, state: S, before: &'a T, after: &'a T) -> Self::Fut;
}

impl<'a, T, Fut, Func, S> StatefulListenerCallback<'a, T, S> for Func
where
    T: Sized + Endpoint + RTObject + Model + 'a,
    S: Sized + Send + Sync,
    Fut: Future + Send,
    Func: Fn(S, &'a T, &'a T) -> Fut,
{
    type Fut = Fut;
    fn call_with_state(&self, state: S, before: &'a T, after: &'a T) -> Self::Fut {
        self(state, before, after)
    }
}

pub trait StatefulNestedListenerCallback<'a, T, S>
where
    T: Sized + Endpoint + RTArray + Model,
    S: Sized + Send + Sync,
{
    type Fut: Future + Send;
    fn call_with_state(&self, state: S, change: Change<'a, T>) -> Self::Fut;
}

impl<'a, T, Fut, Func, S> StatefulNestedListenerCallback<'a, T, S> for Func
where
    T: Sized + Endpoint + RTArray + Model + 'a,
    S: Sized + Send + Sync,
    Fut: Future + Send,
    Func: Fn(S, Change<'a, T>) -> Fut,
{
    type Fut = Fut;
    fn call_with_state(&self, state: S, change: Change<'a, T>) -> Self::Fut {
        self(state, change)
    }
}

pub struct CrossDiff<'a, T>
where
    T: Model + Endpoint + RTArray,
{
    current: &'a [T],
    incoming: &'a [T],
}

impl<'a, T> CrossDiff<'a, T>
where
    T: Model + Endpoint + RTArray,
{
    pub fn new(current: &'a [T], incoming: &'a [T]) -> Self {
        Self { current, incoming }
    }

    pub fn removed(&self) -> Vec<Change<'a, T>> {
        self.current
            .iter()
            .filter(|&item| !self.incoming.contains(item))
            .map(Change::Removed)
            .collect()
    }

    pub fn added(&self) -> Vec<Change<'a, T>> {
        self.incoming
            .iter()
            .filter(|&item| !self.incoming.contains(item))
            .map(Change::Added)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::worldstate::prelude::{Cetus, Fissure};

    use super::Change;

    async fn on_cetus_update(_before: &Cetus, _after: &Cetus) {}
    async fn on_cetus_update_stateful_nested(_state: Arc<i32>, _change: Change<'_, Fissure>) {}

    #[tokio::test]
    async fn test() {
        use crate::worldstate::prelude::*;
        let client = Arc::new(Client::new());

        let cloned = client.clone();
        tokio::task::spawn(async move { cloned.call_on_update(on_cetus_update).await });
        let cloned = client.clone();
        tokio::task::spawn(async move {
            cloned
                .call_on_nested_update_with_state(on_cetus_update_stateful_nested, Arc::new(4))
                .await
        });
    }
}
