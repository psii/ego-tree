#![allow(expl_impl_clone_on_copy)]

use super::NodeId;
use std::hash::{Hash, Hasher};

impl<T> Copy for NodeId<T> { }
impl<T> Clone for NodeId<T> {
    fn clone(&self) -> Self { *self }
}

impl<T> Eq for NodeId<T> { }
impl<T> PartialEq for NodeId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.tree_id == other.tree_id && self.index == other.index
    }
}

impl<T> Hash for NodeId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl<T> NodeId<T> {
    /// Return the underlying index of the node. This is useful for defining
    /// a total order over `NodeId`s.
    pub fn get_wrapped_index(&self) -> usize {
        self.index
    }
}
