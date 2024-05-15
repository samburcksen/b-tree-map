use std::{cmp::Ordering, fmt::Display, usize};

#[derive(Clone)]
pub(crate) struct KeyValuePair<T: Ord + Default + Clone, U: Default + Clone> {
    pub(crate) key: T,
    pub(crate) value: Box<U>
}

impl<T: Ord + Default + Clone, U: Default + Clone> KeyValuePair<T, U> {
    pub(crate) fn new(key: T, value: U) -> Self{
        Self {
            key,
            value: value.into()
        }
    }
}

impl<T: Ord + Default + Clone, U: Default + Clone> Default for KeyValuePair<T, U> {
    fn default() -> Self {
        Self { key: Default::default(), value: Default::default() }
    }
}

#[derive(Clone)]
pub(crate) struct Node<T: Ord + Default + Clone, U: Default + Clone> {
    pub(crate) min_childs: usize,
    pub(crate) elements: Vec<KeyValuePair<T, U>>,
    pub(crate) children: Vec<Node<T, U>>
}

impl<T: Ord + Default + Clone, U: Default + Clone> Node<T, U> {
    pub(crate) fn new(min_childs:usize) -> Self {
        Self {
            min_childs,
            elements: Vec::new(),
            children: Vec::new(),
        }
    }

    pub(crate) fn insert(&mut self, key: T, value: U) {
        let key_pos = self.find_pos(&key);

        // Key is already in Tree
        if key_pos.1 {
            return;
        }

        if self.is_leaf() {
            self.elements.insert(key_pos.0, KeyValuePair::new(key, value));
            return;
        }

        let child = &mut self.children[key_pos.0];

        // If child is full, split it
        if child.is_full() {
            // Get middle key of child
            let middle_key = std::mem::take(&mut child.elements[self.min_childs - 1]);
            let middle_key_pos = self.find_pos(&middle_key.key).0;

            // Split the child and remove the middle key
            let child = &mut self.children[key_pos.0];
            let new_right_child = child.split();

            // Insert middle key into self
            self.elements.insert(middle_key_pos, middle_key);
            self.children.insert(middle_key_pos + 1, new_right_child);
        } 
        
        let key_pos = self.find_pos(&key).0;
        self.children[key_pos].insert(key, value);
    }

    pub(crate) fn split(&mut self) -> Node<T, U> {
        let mut right_sibling = Node::new(self.min_childs);

        // Split keys
        right_sibling.elements.extend_from_slice(&self.elements[self.min_childs..]);
        self.elements = self.elements[..self.min_childs - 1].into();

        // If self is no leaf, split children
        if !self.is_leaf() {
            right_sibling.children.append(&mut self.children[self.min_childs..].into());
            self.children = self.children[..self.min_childs].into();
        }

        right_sibling
    }

    pub(crate) fn remove(&mut self, key: &T) -> Option<U> {
        let mut key_pos = self.find_pos(key);

        // Key is in node
        if key_pos.1 {
            if self.is_leaf() {
                // Remove element and return the removed value
                Some(*self.elements.remove(key_pos.0).value)
            } else {
                self.remove_from_non_leaf(key)
            }

        // Key is not in node
        } else {
            if self.is_leaf() {
                return None;
            }
            
            // Child where key is located has minimal amount of keys
            if self.children[key_pos.0].elements.len() < (self.min_childs) {
                self.fill_child(key_pos.0);

                // Get new key_pos in case child was merged with left node
                key_pos = self.find_pos(key);
            }

            self.children[key_pos.0].remove(key)
        }
    }

    fn remove_from_non_leaf(&mut self, key: &T) -> Option<U> {
        let key_pos = self.find_pos(key).0;
        let min_keys = self.min_childs - 1;

        // Preceeding child has enough keys
        if self.children[key_pos].elements.len() > min_keys {
            // Swap element to be removed with preceeding element in left child 
            let child = &mut self.children[key_pos];
            let preceeding_index = child.elements.len() - 1;
            std::mem::swap(&mut self.elements[key_pos], &mut child.elements[preceeding_index]);

            child.remove(key)

        // Succeeding child has enough keys
        } else if self.children[key_pos + 1].elements.len() > min_keys {
            // Swap element to be removed with succeeding element in right child 
            let child = &mut self.children[key_pos + 1];
            std::mem::swap(&mut self.elements[key_pos], &mut child.elements[0]);
            
            child.remove(key)

        // Both children have minimun amount of keys
        } else {
            self.merge_children(key_pos);
            self.children[key_pos].remove(key)
        }
    }

    fn fill_child(&mut self, pos: usize) {
        let min_keys = self.min_childs - 1;
        let last_child = self.elements.len() == pos;

        // Left sibling exists and has enough keys
        if pos != 0 && self.children[pos - 1].elements.len() > min_keys {
            // Get right-most key and child from left sibling
            let left_sibling = &mut self.children[pos - 1];
            let left_sibling_key = left_sibling.elements.remove(left_sibling.elements.len() - 1);
            if !left_sibling.is_leaf() {
                let left_sibling_child = left_sibling.children.remove(left_sibling.children.len() - 1);
                self.children[pos].children.insert(0, left_sibling_child);
            }

            // Insert left_key into self and overwritten own element into child
            let own_element = std::mem::replace(&mut self.elements[pos - 1], left_sibling_key);
            self.children[pos].elements.insert(0, own_element);

        // Right sibling exists and has enough keys
        } else if pos != self.children.len() - 1 && self.children[pos + 1].elements.len() > min_keys {
            // Get left-most key and child from right sibling
            let right_sibling = &mut self.children[pos + 1];
            let right_sibling_key = right_sibling.elements.remove(0);
            if !right_sibling.is_leaf() {
                let right_sibling_child = right_sibling.children.remove(0);
                self.children[pos].children.push(right_sibling_child);
            }
            
            let own_element = std::mem::replace(&mut self.elements[pos], right_sibling_key);
            self.children[pos].elements.push(own_element);

        // If child is right most child, merge with it's left sibling instead
        } else if last_child{            
            self.merge_children(pos - 1);

        // Otherwise merge the child with the right sibling
        } else {
            self.merge_children(pos);
        }
    }

    fn merge_children(&mut self, left_child_pos: usize) {
        // Remove key from self
        let key = self.elements.remove(left_child_pos);

        // Delete right child
        let mut right_child =  self.children.remove(left_child_pos + 1);

        // Insert key into left child
        let left_child = &mut self.children[left_child_pos];
        left_child.elements.push(key);

        // Move right child keys and children into left child
        left_child.elements.append(&mut right_child.elements);
        left_child.children.append(&mut right_child.children);
    }

    pub(crate) fn find_pos(&self, key: &T) -> (usize, bool) {
        for (index, node_key) in self.elements.iter().enumerate() {
            match key.cmp(&node_key.key) {
                Ordering::Less => return (index, false),
                Ordering::Equal => return (index, true),
                Ordering::Greater => continue
            }
        }

        // Key is larger then all keys
        (self.elements.len(), false)
    }

    pub(crate) fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub(crate) fn is_full(&self) -> bool {
        self.elements.len() >= (self.min_childs * 2 - 1)
    }
}

impl<T: Display + Ord + Default + Clone, U: Default + Clone> Display for Node<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_leaf() {
            for key in &self.elements {
                write!(f, "{}", key.key)?;
            } 
            Ok(())
        } else {
            for (child, key) in self.children.iter().zip(self.elements.iter()) {
                write!(f, "{}{}", child, key.key)?;
            }
            write!(f, "{}", self.children[self.elements.len()])
        }
    }
}
