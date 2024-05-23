use crate::node::Node;

impl<K, V> Node<K, V>
where
    K: Ord
{
    pub(crate) fn search_tree(&self, key: &K) -> Option<&V> {
        let pos = self.find_pos(key);

        // Key is found
        if pos.1 {
            return Some(&self.elements[pos.0].value);
        } 

        // Node is leaf and key was not found in node keys
        if self.is_leaf() {
            return None;
        }  

        self.children[pos.0].search_tree(key)
    }
}