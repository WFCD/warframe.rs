use std::future::Future;

use super::models::{Endpoint, Model, RTArray, RTObject, TimedEvent};

pub enum Change<'a, T> {
    Added(&'a T),
    Removed(&'a T),
}

pub trait ObjectListenerCallback<'a, T>
where
    T: Sized + Endpoint + RTObject + Model + 'a,
{
    fn call(&self, before: &'a T, after: &'a T) -> impl Future + 'a;
}

impl<'a, T, Fut, Func> ObjectListenerCallback<'a, T> for Func
where
    T: Sized + Endpoint + RTObject + Model + 'a,
    Fut: Future + 'a,
    Func: Fn(&'a T, &'a T) -> Fut,
{
    fn call(&self, before: &'a T, after: &'a T) -> impl Future + 'a {
        self(before, after)
    }
}

pub trait ArrayListenerCallback<'a, T>
where
    T: Sized + Endpoint + RTArray + TimedEvent + Model + 'a,
{
    fn call(&self, change: Change<'a, T>) -> impl Future + 'a;
}

impl<'a, T, Fut, Func> ArrayListenerCallback<'a, T> for Func
where
    T: Sized + Endpoint + RTArray + TimedEvent + Model + 'a,
    Fut: Future + 'a,
    Func: Fn(Change<'a, T>) -> Fut,
{
    fn call(&self, change: Change<'a, T>) -> impl Future + 'a {
        self(change)
    }
}
