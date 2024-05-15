use std::fmt::Display;

use crate::node::Node;

pub struct Tree<T: Ord + Default + Clone, U: Default + Clone> {
    min_childs: usize,
    root_node: Node<T, U>
}

impl<T: Ord + Default + Clone, U: Default + Clone> Tree<T, U> {
    pub fn new(min_childs: usize) -> Self {
        if min_childs < 2 {
            panic!("Order must be at least 2");
        }
        Self {
            min_childs, 
            root_node: Node::new(min_childs),
        }
    }

    pub fn insert(&mut self, key: T, value: U) {
        // If root node is  full, split it first
        if self.root_node.is_full() {
            let old_root = &mut self.root_node;

            // Extract the old roots middle key and split it
            let middle_element = std::mem::take(&mut old_root.elements[self.min_childs - 1]);
            let mut right_child =  old_root.split();

            // Insert key into split old root
            if key < middle_element.key {
                old_root.insert(key, value);
            } else {
                right_child.insert(key, value);
            }

            // Move new root into old_root and store old_root in left_child
            let left_child = std::mem::replace( old_root, Node::new(self.min_childs));

            // Insert split old root and its middle key into new root
            self.root_node.children.push(left_child);
            self.root_node.children.push(right_child);
            self.root_node.elements.push(middle_element);
        } else {
            // Insert new key
            self.root_node.insert(key, value);
        }
    }


    pub fn remove(&mut self, key: &T) -> Option<U> {
        if self.root_node.elements.is_empty() {
            return None
        } 

        let removed_value = self.root_node.remove(key);

        // Check if root node still has enough keys
        if self.root_node.elements.is_empty() & !self.root_node.is_leaf() {
            self.root_node = self.root_node.children.remove(0);
        }

        removed_value
    }

    pub fn search(&self, key:  &T) -> Option<&U> {
        let mut node = &self.root_node;

        loop {
            let pos = node.find_pos(key);

            // Key is found
            if pos.1 {
                return Some(&node.elements[pos.0].value);
            } 

            // Node is leaf and key was not found in node keys
            if node.is_leaf() {
                return None;
            }  

            node = &node.children[pos.0];
        }
    }   
}

impl<T: Display + Ord + Default + Clone, U: Default + Clone> Display for Tree<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root_node)
    }
}
