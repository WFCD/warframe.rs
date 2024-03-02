use std::future::Future;

use super::models::{Endpoint, Model, RTArray, RTObject, TimedEvent};

pub enum Change<'a, T> {
    Added(&'a T),
    Removed(&'a T),
}

// ----------
pub trait ListenerCallback<'a, T, S = ()>
where
    T: Sized + Endpoint + RTObject + Model + 'a,
    S: Sized + Send + Sync,
{
    fn call(&self, before: &'a T, after: &'a T) -> impl Future + 'a;
}

impl<'a, T, Fut, Func> ListenerCallback<'a, T> for Func
where
    T: Sized + Endpoint + RTObject + Model + 'a,
    Fut: Future + 'a,
    Func: Fn(&'a T, &'a T) -> Fut,
{
    fn call(&self, before: &'a T, after: &'a T) -> impl Future + 'a {
        self(before, after)
    }
}

pub trait NestedListenerCallback<'a, T>
where
    T: Sized + Endpoint + RTArray + TimedEvent + Model + 'a,
{
    fn call(&self, change: Change<'a, T>) -> impl Future + 'a;
}

impl<'a, T, Fut, Func> NestedListenerCallback<'a, T> for Func
where
    T: Sized + Endpoint + RTArray + TimedEvent + Model + 'a,
    Fut: Future + 'a,
    Func: Fn(Change<'a, T>) -> Fut,
{
    fn call(&self, change: Change<'a, T>) -> impl Future + 'a {
        self(change)
    }
}

// --------- STATEFUL CALLBACKS
pub trait StatefulListenerCallback<'a, T, S>
where
    T: Sized + Endpoint + RTObject + Model + 'a,
    S: Sized + Send + Sync,
{
    fn call_with_state(&self, state: S, before: &'a T, after: &'a T) -> impl Future + 'a;
}

impl<'a, T, Fut, Func, S> StatefulListenerCallback<'a, T, S> for Func
where
    T: Sized + Endpoint + RTObject + Model + 'a,
    S: Sized + Send + Sync,
    Fut: Future + 'a,
    Func: Fn(S, &'a T, &'a T) -> Fut,
{
    fn call_with_state(&self, state: S, before: &'a T, after: &'a T) -> impl Future + 'a {
        self(state, before, after)
    }
}

pub trait StatefulNestedListenerCallback<'a, T, S>
where
    T: Sized + Endpoint + RTArray + TimedEvent + Model + 'a,
{
    fn call_with_state(&self, state: S, change: Change<'a, T>) -> impl Future + 'a;
}

impl<'a, T, Fut, Func, S> StatefulNestedListenerCallback<'a, T, S> for Func
where
    T: Sized + Endpoint + RTArray + TimedEvent + Model + 'a,
    Fut: Future + 'a,
    Func: Fn(S, Change<'a, T>) -> Fut,
{
    fn call_with_state(&self, state: S, change: Change<'a, T>) -> impl Future + 'a {
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
