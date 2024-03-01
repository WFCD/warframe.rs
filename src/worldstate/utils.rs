use crate::ws::{Endpoint, Model, RTArray};

use super::listener::Change;

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
