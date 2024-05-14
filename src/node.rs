use std::{cmp::Ordering, fmt::Display, usize};

use crate::tree::Comparable;

#[derive(Clone)]
pub(crate) struct Node<T: Comparable> {
    pub(crate) order: usize,
    pub(crate) keys: Vec<T>,
    pub(crate) children: Vec<Node<T>>
}

impl<T: Comparable> Node<T> {
    pub(crate) fn new(order:usize) -> Self {
        Self {
            order,
            keys: Vec::new(),
            children: Vec::new(),
        }
    }

    pub(crate) fn insert(&mut self, key: T) {
        let key_pos = self.find_pos(key);

        // Key is already in Tree
        if key_pos.1 {
            return;
        }

        if self.is_leaf() {
            self.keys.insert(key_pos.0, key);
            return;
        }

        let child = &self.children[key_pos.0];

        // If child is full, split it
        if child.is_full() {
            // Get middle key of child
            let middle_key = child.keys[self.order/2 - 1];
            let middle_key_pos = self.find_pos(middle_key).0;

            // Split the child and remove the middle key
            let child = &mut self.children[key_pos.0];
            let new_right_child = child.split();

            // Insert middle key into self
            self.keys.insert(middle_key_pos, middle_key);
            self.children.insert(middle_key_pos + 1, new_right_child);
        } 
        
        let key_pos = self.find_pos(key).0;
        self.children[key_pos].insert(key);
    }

    pub(crate) fn split(&mut self) -> Node<T> {
        let mut right_sibling = Node::new(self.order);

        // Split keys
        right_sibling.keys.extend_from_slice(&self.keys[self.order/2..]);
        self.keys = self.keys[..self.order/2 - 1].into();

        // If self is no leaf, split children
        if !self.is_leaf() {
            right_sibling.children.append(&mut self.children[self.order/2..].into());
            self.children = self.children[..self.order/2].into();
        }

        right_sibling
    }

    pub(crate) fn remove(&mut self, key: T) {
        let mut key_pos = self.find_pos(key);

        // Key is in node
        if key_pos.1 {
            if self.is_leaf() {
                self.keys.remove(key_pos.0);
            } else {
                self.remove_from_non_leaf(key);
            }

        // Key is not in node
        } else {
            if self.is_leaf() {
                return;
            }
            
            // Child where key is located has minimal amount of keys
            if self.children[key_pos.0].keys.len() < (self.order / 2) {
                self.fill_child(key_pos.0);

                // Get new key_pos in case child was merged with left node
                key_pos = self.find_pos(key);
            }

            self.children[key_pos.0].remove(key);
        }
    }

    fn remove_from_non_leaf(&mut self, key: T) {
        let key_pos = self.find_pos(key).0;
        let min_keys = self.order / 2 - 1;

        // Preceeding child has enough keys
        if self.children[key_pos].keys.len() > min_keys {
            // Get preceeding key from left child and remove it recursively
            let left_child = &mut self.children[key_pos];
            let preceeding_key = left_child.keys[left_child.keys.len() - 1];
            left_child.remove(preceeding_key);

            // Replace key with preceeding key from left child
            self.keys[key_pos] = preceeding_key;

        // Succeeding child has enough keys
        } else if self.children[key_pos + 1].keys.len() > min_keys {
            // Get succeeding key from right child and remove it recursively
            let right_child = &mut self.children[key_pos + 1];
            let preceeding_key = right_child.keys[0];
            right_child.remove(preceeding_key);

            // Replace key with preceeding key from left child
            self.keys[key_pos] = preceeding_key;

        // Both children have minimun amount of keys
        } else {
            self.merge_children(key_pos);
            self.children[key_pos].remove(key);
        }

    }

    fn fill_child(&mut self, pos: usize) {
        let min_keys = self.order / 2 - 1;
        let last_child = self.keys.len() == pos;

        // Left sibling exists and has enough keys
        if pos != 0 && self.children[pos - 1].keys.len() > min_keys {
            // Get right-most key and child from left sibling
            let left_sibling = &mut self.children[pos - 1];
            let left_sibling_key = left_sibling.keys.remove(left_sibling.keys.len() - 1);
            if !left_sibling.is_leaf() {
                let left_sibling_child = left_sibling.children.remove(left_sibling.children.len() - 1);
                self.children[pos].children.insert(0, left_sibling_child);
            }

            // Insert left_key into self and overwritten own key into child
            self.children[pos].keys.insert(0, self.keys[pos - 1]);
            self.keys[pos - 1] = left_sibling_key;

        // Right sibling exists and has enough keys
        } else if pos != self.children.len() - 1 && self.children[pos + 1].keys.len() > min_keys {
            // Get left-most key and child from right sibling
            let right_sibling = &mut self.children[pos + 1];
            let right_sibling_key = right_sibling.keys.remove(0);
            if !right_sibling.is_leaf() {
                let right_sibling_child = right_sibling.children.remove(0);
                self.children[pos].children.push(right_sibling_child);
            }
            
            self.children[pos].keys.push(self.keys[pos]);
            self.keys[pos] = right_sibling_key;

        // If child is right most child, merge with it's left sibling instead
        } else if last_child{            
            self.merge_children(pos - 1);

        // Otherwise merge the child with the right sibling
        } else {
            self.merge_children(pos);
        }
    }

    fn merge_children(&mut self, left_child: usize) {
        // Remove key from self
        let key = self.keys.remove(left_child);

        // Delete right child
        let mut right_child =  self.children.remove(left_child + 1);

        // Insert key into left child
        let left_child = &mut self.children[left_child];
        left_child.keys.push(key);

        // Move right child keys and children into left child
        left_child.keys.append(&mut right_child.keys);
        left_child.children.append(&mut right_child.children);
    }

    pub(crate) fn find_pos(&self, key: T) -> (usize, bool) {
        for (index, node_key) in self.keys.iter().enumerate() {
            match key.cmp(node_key) {
                Ordering::Less => return (index, false),
                Ordering::Equal => return (index, true),
                Ordering::Greater => continue
            }
        }

        // Key is larger then all keys
        (self.keys.len(), false)
    }

    pub(crate) fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub(crate) fn is_full(&self) -> bool {
        self.keys.len() >= (self.order - 1)
    }
}

impl<T: Display + Comparable> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_leaf() {
            for key in &self.keys {
                write!(f, "{}", key)?;
            } 
            Ok(())
        } else {
            for (child, key) in self.children.iter().zip(self.keys.iter()) {
                write!(f, "{}{}", child, key)?;
            }
            write!(f, "{}", self.children[self.keys.len()])
        }
    }
}
