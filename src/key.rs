use std::fmt::Display;

use crate::node::NodePtr;

#[derive(Clone)]
pub struct Key<T: Ord + Clone + Display> {
    pub value: T,
    pub left: Option<NodePtr<T>>,
    pub right: Option<NodePtr<T>>,
}

impl<T: Ord + Clone + Display> Key<T> {
    pub fn new(value: T, left: Option<NodePtr<T>>, right: Option<NodePtr<T>>) -> Self {
        Self { value, left, right }
    }
}
