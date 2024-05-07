use core::panic;

use crate::tree::Comparable;

#[derive(Clone)]
pub struct Node<T: Comparable> {
    order: usize,
    keys: Vec<T>,
    children: Vec<Node<T>>
}

impl<T: Comparable> Node<T> {
    pub fn new(order:usize) -> Self {
        Self {
            order,
            keys: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: T) {
        let key_pos = self.find_pos(key);

        // Key is already in Tree
        if key_pos.1 {
            return;
        }

        if self.is_leaf() {
            self.keys.insert(key_pos.0, key);
            return;
        }

        let child = self.child(key_pos.0).unwrap();

        // If child is full, split it
        if child.is_full() {
            // Get middle key of child
            let middle_key = *child.keys().get(self.order/2 - 1).unwrap();
            let middle_key_pos = self.find_pos(middle_key).0;

            // Split the child and remove the middle key
            let child = self.child_mut(key_pos.0).unwrap();
            let new_right_child = child.split();

            // Insert middle key into self
            self.keys.insert(middle_key_pos, middle_key);
            self.children.insert(middle_key_pos + 1, new_right_child);
        } 
        
        let child = self.child_mut(self.find_pos(key).0).unwrap();
        child.insert(key);

    }

    pub fn split(&mut self) -> Node<T> {
        let mut right_sibling = Node::new(self.order);

        // Split keys
        right_sibling.keys_mut().append(&mut self.keys[self.order/2..].into());
        self.keys = self.keys[..self.order/2 - 1].into();

        // If self is no leaf, split children
        if !self.is_leaf() {
            right_sibling.children_mut().append(&mut self.children[self.order/2..].into());
            self.children = self.children[..self.order/2].into();
        }

        right_sibling
    }

    pub fn find_pos(&self, key: T) -> (usize, bool) {
        for (index, node_key) in self.keys.iter().enumerate() {
            if key < *node_key {
                return (index, false);
            } else if key == *node_key {
                return (index, true);
            }
        }

        // Key is larger then all keys
        (self.keys.len(), false)
    }

    pub fn keys(&self) -> &Vec<T> {
        &self.keys
    }

    pub fn keys_mut(&mut self) -> &mut Vec<T> {
        &mut self.keys
    }

    pub fn children_mut(&mut self) -> &mut Vec<Node<T>> {
        &mut self.children
    } 

    pub fn child(&self, index: usize) -> Option<&Node<T>> {
        self.children.get(index)
    }

    pub fn child_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
        self.children.get_mut(index)
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.keys.len() >= (self.order - 1)
    }

    pub fn print(&self) {
        if self.is_leaf() {
            for key in &self.keys {
                print!("{}", key);
            }
        } else {
            for (index, key) in self.keys.iter().enumerate() {
                self.children[index].print();
                print!("{}", key);
            }
            self.children[self.keys.len()].print();
        }
    }
}
