use crate::node::Node;

impl<K, V> Node<K, V>
where
    K: Ord,
{
    pub(crate) fn remove(&mut self, key: &K) -> Option<V> {
        let mut key_pos = self.find_pos(key);

        // Key is in node
        if key_pos.1 {
            if self.is_leaf() {
                // Remove element and return the removed value
                Some(self.elements.remove(key_pos.0).value)
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

    fn remove_from_non_leaf(&mut self, key: &K) -> Option<V> {
        let key_pos = self.find_pos(key).0;
        let min_keys = self.min_childs - 1;

        // Preceeding child has enough keys
        if self.children[key_pos].elements.len() > min_keys {
            // Swap element to be removed with preceeding element in left child
            let child = &mut self.children[key_pos];
            let preceeding_index = child.elements.len() - 1;
            std::mem::swap(
                &mut self.elements[key_pos],
                &mut child.elements[preceeding_index],
            );

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
            let left_sibling_key = left_sibling
                .elements
                .remove(left_sibling.elements.len() - 1);
            if !left_sibling.is_leaf() {
                let left_sibling_child = left_sibling
                    .children
                    .remove(left_sibling.children.len() - 1);
                self.children[pos].children.insert(0, left_sibling_child);
            }

            // Insert left_key into self and overwritten own element into child
            let own_element = std::mem::replace(&mut self.elements[pos - 1], left_sibling_key);
            self.children[pos].elements.insert(0, own_element);

        // Right sibling exists and has enough keys
        } else if pos != self.children.len() - 1 && self.children[pos + 1].elements.len() > min_keys
        {
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
        } else if last_child {
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
        let mut right_child = self.children.remove(left_child_pos + 1);

        // Insert key into left child
        let left_child = &mut self.children[left_child_pos];
        left_child.elements.push(key);

        // Move right child keys and children into left child
        left_child.elements.append(&mut right_child.elements);
        left_child.children.append(&mut right_child.children);
    }
}
