use std::future::Future;

use super::models::{Endpoint, Model, RTArray, RTObject};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Change {
    Added,
    Removed,
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
    fn call(&self, item: &'a T, change: Change) -> Self::Fut;
}

impl<'a, T, Fut, Func> NestedListenerCallback<'a, T> for Func
where
    T: Sized + Endpoint + RTArray + Model + 'a,
    Fut: Future + Send,
    Func: Fn(&'a T, Change) -> Fut,
{
    type Fut = Fut;
    fn call(&self, item: &'a T, change: Change) -> Self::Fut {
        self(item, change)
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
    fn call_with_state(&self, state: S, item: &'a T, change: Change) -> Self::Fut;
}

impl<'a, T, Fut, Func, S> StatefulNestedListenerCallback<'a, T, S> for Func
where
    T: Sized + Endpoint + RTArray + Model + 'a,
    S: Sized + Send + Sync,
    Fut: Future + Send,
    Func: Fn(S, &'a T, Change) -> Fut,
{
    type Fut = Fut;
    fn call_with_state(&self, state: S, item: &'a T, change: Change) -> Self::Fut {
        self(state, item, change)
    }
}

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
    pub fn new(current: &'a [T], incoming: &'a [T]) -> Self {
        Self { current, incoming }
    }

    pub fn removed(&self) -> Vec<(&'a T, Change)> {
        self.current
            .iter()
            .filter(|&item| !self.incoming.contains(item))
            .map(|item| (item, Change::Removed))
            .collect()
    }

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
    use std::{sync::Arc, vec};

    use crate::worldstate::prelude::{Cetus, Fissure};

    use super::{Change, CrossDiff};

    async fn on_cetus_update(_before: &Cetus, _after: &Cetus) {}
    async fn on_cetus_update_stateful_nested(_state: Arc<i32>, _item: &Fissure, _change: Change) {}

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

    #[test]
    fn test_crossdiff() {
        let a = vec![1, 2, 3];
        let b = vec![1, 3, 4];
        let cf = CrossDiff::new(&a, &b);
        assert_eq!(cf.added(), vec![(&4, Change::Added)]);
        assert_eq!(cf.removed(), vec![(&2, Change::Removed)]);
    }
}
