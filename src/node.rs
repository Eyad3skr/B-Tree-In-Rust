use crate::key::Key;
use std::fmt::Display;
use std::fs::File;
use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Clone)]
pub enum NodeType {
    Root,
    Internal,
    Leaf,
}

pub type NodePtr<T> = Rc<RefCell<Node<T>>>;

#[derive(Clone)]
pub struct Node<T: Ord + Clone + Display> {
    pub node_type: NodeType,
    pub keys_vector: Vec<Key<T>>,
}

impl<T: Ord + Clone + Display> Node<T> {
    pub fn new(node_type: NodeType, keys_vector: Vec<Key<T>>) -> Self {
        Self {
            node_type,
            keys_vector,
        }
    }

    // pub fn insert(&mut self, value: T) -> Option<NodePtr<T>> {
    //     if !self.has_children() {
    //         let new_key = Key {
    //             value,
    //             left: None,
    //             right: None,
    //         };
    //         self.keys_vector.push(new_key);
    //         // TODO : handle overflow
    //         return None;
    //     }
    //
    //     self.get_next(&value).borrow_mut().insert(value)
    // }

    pub fn search(&self, target: T) -> bool {
        let found = self.has_key(&target);

        if found {
            return true;
        }

        if self.node_type == NodeType::Leaf || !self.has_children() {
            return false;
        }

        self.get_next(&target).borrow().search(target)
    }
    //Return the child pointer we should descend into for `target`.
    pub fn get_next(&self, target: &T) -> NodePtr<T> {
        if let Some(k) = self.keys_vector.iter().find(|k| target < &k.value) {
            return std::rc::Rc::clone(
                k.left
                    .as_ref()
                    .expect("malformed B-Tree: missing left child"),
            );
        }
        std::rc::Rc::clone(
            self.keys_vector
                .last()
                .and_then(|k| k.right.as_ref())
                .expect("malformed B-Tree: missing right child on last key"),
        )
    }

    pub fn leaf_insert_sorted(&mut self, value: T) {
        match self.keys_vector.binary_search_by(|k| k.value.cmp(&value)) {
            Ok(_) => { /* duplicate policy: ignore */ }
            Err(i) => self.keys_vector.insert(
                i,
                Key {
                    value,
                    left: None,
                    right: None,
                },
            ),
        }
    }
    pub fn insert_key_sorted(&mut self, key: Key<T>) {
        match self
            .keys_vector
            .binary_search_by(|k| k.value.cmp(&key.value))
        {
            Ok(_) => { /* duplicate policy: ignore */ }
            Err(i) => {
                self.keys_vector.insert(i, key.clone());
                let left_child = Rc::clone(&key.left.unwrap());
                let right_child = Rc::clone(&key.right.unwrap());

                if (i as isize - 1) >= 0 {
                    self.keys_vector[i - 1].right = Some(left_child);
                }

                if i + 1 <= &self.keys_vector.len() - 1 {
                    self.keys_vector[i + 1].left = Some(right_child);
                }
            }
        }
    }
    // bool function
    pub fn has_key(&self, target: &T) -> bool {
        for key in &self.keys_vector {
            if key.value == *target {
                return true;
            }
        }
        false
    }

    // Helper function used in insert and search to check if the node has children
    pub fn has_children(&self) -> bool {
        if let Some(last) = self.keys_vector.last() {
            if last.right.is_some() {
                return true;
            }
        }
        self.keys_vector.iter().any(|k| k.left.is_some())
    }

    /// Collect children in B-Tree order: left of each key, then last right (if any).
    pub fn collect_children(&self) -> Vec<NodePtr<T>> {
        let mut ch = Vec::new();
        for k in &self.keys_vector {
            if let Some(l) = &k.left {
                ch.push(std::rc::Rc::clone(l));
            }
        }
        let last = self.keys_vector.last().unwrap();

        // if you got a runtime error here, unconmment the working code, and commment out your
        // shitty code
        //
        //if let Some(last) = self.keys_vector.last() {
        if let Some(r) = &last.right {
            ch.push(std::rc::Rc::clone(r));
        }
        //}
        ch
    }
}
