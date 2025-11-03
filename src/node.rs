use crate::key::Key;
use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Clone)]
pub enum NodeType {
    Root,
    Internal,
    Leaf,
}

pub type NodePtr<T> = Rc<RefCell<Node<T>>>;

#[derive(Clone)]
pub struct Node<T: Ord + Clone> {
    pub node_type: NodeType,
    pub keys_vector: Vec<Key<T>>,
}

impl<T: Ord + Clone> Node<T> {
    pub fn new(node_type: NodeType, keys_vector: Vec<Key<T>>) -> Self {
        Self {
            node_type,
            keys_vector,
        }
    }

    pub fn insert(&mut self, value: T) -> Option<NodePtr<T>> {
        if self.node_type == NodeType::Leaf || !self.has_children() {
            let new_key = Key {
                value,
                left: None,
                right: None,
            };
            self.keys_vector.push(new_key);
            // TODO : handle overflow
            return None;
        }

        self.get_next(&value).borrow_mut().insert(value)
    }

    pub fn search(&self, target: T) -> bool {
        let found = self.find_key(&target);

        if found {
            return true;
        }

        if self.node_type == NodeType::Leaf || !self.has_children() {
            return false;
        }

        self.get_next(&target).borrow().search(target)
    }

    fn get_next(&self, target: &T) -> NodePtr<T> {
        for key in &self.keys_vector {
            if *target < key.value {
                return Rc::clone(&(key.left.as_ref().unwrap()));
            }
        }
        Rc::clone(
            &(self.keys_vector[self.keys_vector.len() - 1]
                .right
                .as_ref()
                .unwrap()),
        )
    }

    fn find_key(&self, target: &T) -> bool {
        for key in &self.keys_vector {
            if key.value == *target {
                return true;
            }
        }
        false
    }
    fn has_children(&self) -> bool {
        for key in &self.keys_vector {
            if let Some(_) = key.right {
                return true;
            }
            if let Some(_) = key.left {
                return true;
            }
        }
        false
    }
}
