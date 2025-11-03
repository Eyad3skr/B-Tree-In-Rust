//Basic B Tree Implementation in Rust
//CRUD Operations: Insert, Search, Delete
/*
* We determine the maximum values in a node first as key, and the min is max/2.
*  -> max number of children = size of node + 1
*  -> max = k, min = k/1. you can only merge with a node of size k/2
* search:
*   - check whether the value is stored on the current node
*   - if not, we check for which range in our list does the value fall
*   - navigate to the chosen node using it is pointer linked to the value for a certain range
*   - repeat
*
* insertion:
*   - navigate to the leaf (using same search algorithm)
*   - check whether it is full (regarding size) or not
*   - if not full insert it and order the list of values
*   - if full:
*       a - split into two nodes.
*       b - take the median element, raise it up as a separator in the parent node.
*       c - check recursively if the element we rose caused a conflict.
*
* Deletion:
*   - search for the element and navigate
*   - if leaf, delete and handle underflow
*   - else, choose node successor or predecessor and replace the deleted value with it
*   - handle underflow in the node you used its successor/predecessor
*
* Underflow handling:
*   - if one of the node siblings size's are greater than minimum, borrow from it.
*   - borrow technique:
*       a - choose right sibling min value or left sibgling max values.
*       b - replace the chosen value to be the separator and take the old separator to be in the
*       underflowed node.
*   - if there ain't a sibling with such specification we megre
*   - merging technique:
*       a - form new node, with your values + separator + values of node you are merging with,
*       according to the rule (max = k, min = k/1. you can only merge with a node of size
*       k/2).
*
*/

pub mod btree;
pub mod key;
pub mod node;

use btree::BTree;

fn main() {
    let mut btree: BTree<usize> = BTree::new(4);
    btree.insert(20);
    println!("{}", btree.search(20));
    println!("{}", btree.search(21));
}
