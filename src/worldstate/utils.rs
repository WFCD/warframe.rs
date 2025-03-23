/// Represents what has happened to the nested Item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Change {
    /// The Item has been added to the collection
    Added,
    /// The Item has been removed the collection
    Removed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CrossDiff<'a, T> {
    current: &'a [T],
    incoming: &'a [T],
}

impl<'a, T> CrossDiff<'a, T>
where
    T: PartialEq,
{
    /// Creates a [`CrossDiff`]
    pub fn new(current: &'a [T], incoming: &'a [T]) -> Self {
        Self { current, incoming }
    }

    /// Gets all the removed items
    #[must_use]
    pub fn removed(&self) -> Vec<(&'a T, Change)> {
        self.current
            .iter()
            .filter(|&item| !self.incoming.contains(item))
            .map(|item| (item, Change::Removed))
            .collect()
    }

    /// Gets all the added items
    #[must_use]
    pub fn added(&self) -> Vec<(&'a T, Change)> {
        self.incoming
            .iter()
            .filter(|&item| !self.current.contains(item))
            .map(|item| (item, Change::Added))
            .collect()
    }
}
