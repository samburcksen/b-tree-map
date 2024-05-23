use crate::{map::BTreeMap, node::{KeyValuePair, Node}};

impl<K,V> BTreeMap<K, V>
where 
    K: Ord + Clone + Default,
    V: Clone + Default
{
    pub(crate) fn split_root(&mut self) {
        let old_root = &mut self.root_node;

            // Extract the old roots middle key and split it
            let middle_element = std::mem::take(&mut old_root.elements[self.min_childs - 1]);
            let right_child =  old_root.split();

            // Move new root into old_root and store old_root in left_child
            let left_child = std::mem::replace( old_root, Node::new(self.min_childs));

            // Insert split old root and its middle key into new root
            self.root_node.children.push(left_child);
            self.root_node.children.push(right_child);
            self.root_node.elements.push(middle_element);
    }
}

impl<K, V> Node<K, V> 
where
    K: Ord + Clone + Default,
    V: Clone + Default
{
    pub(crate) fn insert(&mut self, key: K, value: V) -> Option<V> {
        let key_pos = self.find_pos(&key);

        // Key is already in Tree
        if key_pos.1 {
            // Replace value and return old value
            return Some(std::mem::replace(
                &mut self.elements[key_pos.0].value, 
                value)
            );
        }

        if self.is_leaf() {
            self.elements.insert(key_pos.0, KeyValuePair::new(key, value));
            return None;
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
        self.children[key_pos].insert(key, value)
    }

    pub(crate) fn split(&mut self) -> Node<K, V> {
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
}