use std::fmt::Display;

use crate::node::Node;

pub trait Comparable: PartialEq + Eq + PartialOrd + Ord + Display + Copy {}

impl<C> Comparable for C 
    where C: PartialEq + Eq + PartialOrd + Ord + Display+ Copy 
{}


pub struct Tree<T: Comparable> {
    order: usize,
    root_node: Node<T>
}

impl<T: Comparable> Tree<T> {
    pub fn new(order: usize) -> Self {
        Self {
            order, 
            root_node: Node::new(order),
        }
    }

    pub fn insert(&mut self, key: T) {
        // If root node is  full, split it first
        if self.root_node.is_full() {
            let left_child = &mut self.root_node;

            // Extrace the old roots middle key and split it
            let middle_key = left_child.keys()[self.order/2 - 1];
            let mut right_child =  left_child.split();

            // Insert key into split old root
            if key < middle_key {
                left_child.insert(key);
            } else {
                right_child.insert(key);
            }

            // Move new root into old_root and store old_root in left_child
            let left_child = std::mem::replace( left_child, Node::new(self.order));

            // Insert split old root and its middle key into new root
            self.root_node.children_mut().push(left_child);
            self.root_node.children_mut().push(right_child);
            self.root_node.keys_mut().push(middle_key);
        } else {
            // Insert new key
            self.root_node.insert(key);
        }
    }


    pub fn remove(&mut self, key: T) {

    }

    pub fn search(&self, key:  T) -> Option<&Node<T>> {
        let mut node = &self.root_node;

        loop {
            let pos = node.find_pos(key);

            // Key is found
            if pos.1 {
                return node.into();
            } 

            // Node is leaf and key was not found in node keys
            if node.is_leaf() {
                return None;
            }  

            node = node.child(pos.0).unwrap();
        }
    }

    pub fn print(&self) {
        self.root_node.print();
        println!("");
    }
    
}