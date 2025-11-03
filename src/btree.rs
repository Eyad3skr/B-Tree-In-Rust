use crate::node::Node;
use crate::node::NodePtr;
use crate::node::NodeType;
use std::cell::RefCell;
use std::rc::Rc;

pub struct BTree<T: Ord + Clone> {
    pub degree: usize,
    pub root: NodePtr<T>,
}

impl<T: Ord + Clone> BTree<T> {
    pub fn new(degree: usize) -> Self {
        Self {
            degree,
            root: Rc::new(RefCell::new(Node::new(NodeType::Root, vec![]))),
        }
    }
    pub fn insert(&mut self, value: T) {
        let optional_new_root = self.root.borrow_mut().insert(value);
        if let Some(valid_new_root) = optional_new_root {
            self.root = valid_new_root;
        }
    }

    pub fn search(&self, target: T) -> bool {
        self.root.borrow().search(target)
    }

    pub fn delete(&self, _value: T) -> std::io::Result<()> {
        todo!()
    }
}
