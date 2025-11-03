use crate::node::NodePtr;

#[derive(Clone)]
pub struct Key<T: Ord + Clone> {
    pub value: T,
    pub left: Option<NodePtr<T>>,
    pub right: Option<NodePtr<T>>,
}
