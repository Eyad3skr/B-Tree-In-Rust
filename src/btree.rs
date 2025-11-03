use crate::key::Key;
use crate::node::Node;
use crate::node::NodePtr;
use crate::node::NodeType;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

pub struct BTree<T: Ord + Clone + Display> {
    pub degree: usize,
    pub root: NodePtr<T>,
}

impl<T: Ord + Clone + Display> BTree<T> {
    fn max_keys(&self) -> usize {
        2 * self.degree - 1
    }
    //fn min_keys(&self) -> usize {
    //    self.degree - 1
    //}

    pub fn new(degree: usize) -> Self {
        Self {
            degree,
            root: Rc::new(RefCell::new(Node::new(NodeType::Root, vec![]))),
        }
    }

    pub fn insert(&mut self, value: T) {
        let mut target_leaf: NodePtr<T> = Rc::clone(&self.root);
        let mut path: Vec<NodePtr<T>> = Vec::new();

        path.push(Rc::clone(&target_leaf));

        while target_leaf.borrow().has_children() {
            target_leaf = Rc::clone(&target_leaf).borrow().get_next(&value);
            path.push(Rc::clone(&target_leaf));
        }

        target_leaf.borrow_mut().leaf_insert_sorted(value);
        self.handle_overflow(path);
    }

    fn handle_overflow(&mut self, nodes: Vec<NodePtr<T>>) {
        for i in (0..(nodes.len())).rev() {
            if nodes[i].borrow().keys_vector.len() < self.max_keys() {
                return;
            }

            // split into left node - median key - right node
            let curr_node = Rc::clone(&nodes[i]);
            let median_key_value = curr_node.borrow().keys_vector[self.degree - 1]
                .value
                .clone();

            let mut new_left_keys: Vec<Key<T>> = Vec::new();
            for i in 0..self.degree - 1 {
                new_left_keys.push(curr_node.borrow().keys_vector[i].clone());
            }

            let new_left_node = Rc::new(RefCell::new(Node::new(
                curr_node.borrow().node_type.clone(),
                new_left_keys,
            )));

            let mut new_right_keys: Vec<Key<T>> = Vec::new();
            for i in self.degree..self.max_keys() {
                new_right_keys.push(curr_node.borrow().keys_vector[i].clone());
            }

            let new_right_node = Rc::new(RefCell::new(Node::new(
                curr_node.borrow().node_type.clone(),
                new_right_keys,
            )));

            let new_separator_key =
                Key::new(median_key_value, Some(new_left_node), Some(new_right_node));

            if i != 0 {
                nodes[i - 1]
                    .borrow_mut()
                    .insert_key_sorted(new_separator_key);
            } else {
                curr_node.borrow_mut().node_type = NodeType::Internal;
                let new_root = Node::new(NodeType::Root, vec![new_separator_key]);
                self.root = Rc::new(RefCell::new(new_root));
            }
        }
    }

    //pub fn insert(&mut self, value: T) {
    //    if self.root.borrow().keys_vector.len() == self.max_keys() {
    //        let old_root = Rc::clone(&self.root);
    //        let new_root_ptr = Rc::new(RefCell::new(Node::new(
    //            NodeType::Internal,
    //            vec![Key {
    //                value: value.clone(),
    //                left: Some(old_root),
    //                right: None,
    //            }],
    //        )));
    //        self.root = Rc::clone(&new_root_ptr);
    //
    //        self.split_child_left_of(Rc::clone(&new_root_ptr), 0);
    //
    //        // remove the temporary bootstrap key
    //        {
    //            let mut nr = new_root_ptr.borrow_mut();
    //            if nr.keys_vector.len() >= 2
    //                && nr.keys_vector[1].left.is_some()
    //                && nr.keys_vector[1].right.is_none()
    //            {
    //                nr.keys_vector.remove(1);
    //            }
    //        }
    //
    //        self.insert_non_full(new_root_ptr, value);
    //        self.root.borrow_mut().node_type = NodeType::Root;
    //    } else {
    //        let r = Rc::clone(&self.root);
    //        self.insert_non_full(r, value);
    //    }
    //}

    fn insert_non_full(&self, node: NodePtr<T>, value: T) {
        let mut nb = node.borrow_mut();

        if nb.node_type == NodeType::Leaf || !nb.has_children() {
            nb.leaf_insert_sorted(value);
            return;
        }

        // choose child slot
        enum Slot {
            LeftOf(usize),
            RightOfLast,
        }
        let (slot, child) = {
            if let Some((i, _)) = nb
                .keys_vector
                .iter()
                .enumerate()
                .find(|(_, k)| value < k.value)
            {
                let c = std::rc::Rc::clone(nb.keys_vector[i].left.as_ref().unwrap());
                (Slot::LeftOf(i), c)
            } else {
                let c = std::rc::Rc::clone(
                    nb.keys_vector
                        .last()
                        .and_then(|k| k.right.as_ref())
                        .unwrap(),
                );
                (Slot::RightOfLast, c)
            }
        };

        // split child if full
        if child.borrow().keys_vector.len() == self.max_keys() {
            drop(nb);
            match slot {
                Slot::LeftOf(i) => self.split_child_left_of(std::rc::Rc::clone(&node), i),
                Slot::RightOfLast => self.split_child_right_of_last(std::rc::Rc::clone(&node)),
            }

            // after split, decide side by comparing to promoted separator
            let p = node.borrow_mut();
            match slot {
                Slot::LeftOf(i) => {
                    let sep = p.keys_vector[i].value.clone();
                    let next = if value < sep {
                        std::rc::Rc::clone(p.keys_vector[i].left.as_ref().unwrap())
                    } else {
                        std::rc::Rc::clone(p.keys_vector[i].right.as_ref().unwrap())
                    };
                    drop(p);
                    self.insert_non_full(next, value);
                }
                Slot::RightOfLast => {
                    let j = p.keys_vector.len() - 1;
                    let sep = p.keys_vector[j].value.clone();
                    let next = if value < sep {
                        std::rc::Rc::clone(p.keys_vector[j].left.as_ref().unwrap())
                    } else {
                        std::rc::Rc::clone(p.keys_vector[j].right.as_ref().unwrap())
                    };
                    drop(p);

                    self.insert_non_full(next, value);
                }
            }
        } else {
            drop(nb);
            self.insert_non_full(child, value);
        }
    }
    // Split the child referenced by parent.keys[i].left
    fn split_child_left_of(&self, parent: NodePtr<T>, i: usize) {
        let t = self.degree;

        let child_ptr = {
            let p = parent.borrow();
            std::rc::Rc::clone(p.keys_vector[i].left.as_ref().expect("missing left"))
        };

        let mut y = child_ptr.borrow_mut();
        let is_leaf = y.node_type == NodeType::Leaf || !y.has_children();

        let median_val = y.keys_vector[t - 1].value.clone();
        let right_keys = y.keys_vector.split_off(t);
        let _ = y.keys_vector.pop().expect("pop median");

        let z = Node::new(
            if is_leaf {
                NodeType::Leaf
            } else {
                NodeType::Internal
            },
            right_keys,
        );
        let z_ptr = std::rc::Rc::new(std::cell::RefCell::new(z));
        let left_ptr = std::rc::Rc::clone(&child_ptr);
        drop(y);

        let promoted = Key {
            value: median_val,
            left: Some(left_ptr),
            right: Some(z_ptr),
        };

        let mut p = parent.borrow_mut();
        p.keys_vector.insert(i, promoted);
        if p.node_type == NodeType::Root {
            p.node_type = NodeType::Internal;
        }
    }

    // Split the child that sits in the "right-of-last" position
    fn split_child_right_of_last(&self, parent: NodePtr<T>) {
        let t = self.degree;

        let child_ptr = {
            let p = parent.borrow();
            std::rc::Rc::clone(
                p.keys_vector
                    .last()
                    .and_then(|k| k.right.as_ref())
                    .expect("missing last right"),
            )
        };

        let mut y = child_ptr.borrow_mut();
        let is_leaf = y.node_type == NodeType::Leaf || !y.has_children();

        let median_val = y.keys_vector[t - 1].value.clone();
        let right_keys = y.keys_vector.split_off(t);
        let _ = y.keys_vector.pop().expect("pop median");

        let z = Node::new(
            if is_leaf {
                NodeType::Leaf
            } else {
                NodeType::Internal
            },
            right_keys,
        );
        let z_ptr = std::rc::Rc::new(std::cell::RefCell::new(z));
        let left_ptr = std::rc::Rc::clone(&child_ptr);
        drop(y);

        let promoted = Key {
            value: median_val,
            left: Some(left_ptr),
            right: Some(z_ptr),
        };

        let mut p = parent.borrow_mut();
        p.keys_vector.push(promoted);
        if p.node_type == NodeType::Root {
            p.node_type = NodeType::Internal;
        }
    }

    pub fn search(&self, target: T) -> bool {
        self.root.borrow().search(target)
    }

    pub fn delete(&self, _value: T) -> std::io::Result<()> {
        todo!()
    }

    /// Pretty, boxed tree printer (Unicode).
    /// Hierarchical tree printer with edge labels:
    /// - Root line ends with "(root)"
    /// - Children lines end with "(L of k<i>)" or "(R of k<j>)"
    pub fn print_pretty(&self)
    where
        T: std::fmt::Display,
    {
        let mut out = String::new();
        self.fmt_tree_annotated(&self.root, "", true, "(root)", &mut out);
        println!("{out}");
    }

    fn fmt_tree_annotated(
        &self,
        node: &NodePtr<T>,
        prefix: &str,
        is_tail: bool,
        from_edge: &str, // e.g., "(L of k0)" / "(R of k2)" / "(root)"
        out: &mut String,
    ) where
        T: std::fmt::Display,
    {
        use std::fmt::Write;

        let n = node.borrow();

        // Render this node's keys like [a,b,c]
        let mut label = String::from("[");
        for (i, k) in n.keys_vector.iter().enumerate() {
            if i > 0 {
                label.push(',');
            }
            write!(&mut label, "{}", k.value).unwrap();
        }
        label.push(']');

        // Current line
        let branch = if prefix.is_empty() {
            ""
        } else if is_tail {
            "└── "
        } else {
            "├── "
        };
        let _ = writeln!(out, "{}{}{} {}", prefix, branch, label, from_edge);

        // Build ordered children with edge annotations:
        // for each key i: left child (if any), then after the loop the last key's right child (if any)
        let mut children: Vec<(NodePtr<T>, String)> = Vec::new();
        for (i, k) in n.keys_vector.iter().enumerate() {
            if let Some(l) = &k.left {
                children.push((std::rc::Rc::clone(l), format!("(L of k{i})")));
            }
        }
        if let Some(last_i) = n.keys_vector.len().checked_sub(1) {
            if let Some(r) = n.keys_vector[last_i].right.as_ref() {
                children.push((std::rc::Rc::clone(r), format!("(R of k{last_i})")));
            }
        }
        drop(n); // release before recursing

        if !children.is_empty() {
            let child_prefix = if prefix.is_empty() {
                String::new()
            } else if is_tail {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            let children_clone = children.clone();
            for (i, (ch, edge_lbl)) in children.into_iter().enumerate() {
                let last = i + 1 == children_clone.len();
                self.fmt_tree_annotated(&ch, &child_prefix, last, &edge_lbl, out);
            }
        }
    }
}
