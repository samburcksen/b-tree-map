use crate::node::{Node, Root};

pub struct Tree<T: PartialEq + Eq + PartialOrd + Ord, U> {
    order: u32,
    root_node: Root<T, U>,
}

impl<T: PartialEq + Eq + PartialOrd + Ord, U> Tree<T, U> {
    pub fn new(order: u32) -> Self {
        Self {
            order, 
            root_node: Root::new()
        }
    }

    pub fn insert(&mut self, key: T, value: U) {

    }

    pub fn remove(&mut self, key: T) {

    }

    pub fn get(&self, key:  T) /*-> &U*/ {
        
    }
    
}