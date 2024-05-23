use std::fmt::Display;

use crate::node::Node;

const DEFAULT_ORDER: usize = 6;

pub struct BTreeMap<
    K,
    V
> {
    pub(super) min_childs: usize,
    pub(super) root_node: Node<K, V>
}

impl<K,V> BTreeMap<K, V> {

    pub fn new() -> Self {
        Self::with_order(DEFAULT_ORDER).unwrap()
    }

    pub fn with_order(min_childs: usize) -> Result<Self, &'static str> {
        if min_childs < 2 {
            return Err("min_childs needs to be larger than 2!")
        }

        Ok(Self {
            min_childs,
            root_node: Node::new(min_childs)
        })
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord + Clone + Default,
        V: Clone + Default
    {
        if self.root_node.is_full() {
            self.split_root();
        } 

        self.root_node.insert(key, value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V>
    where
        K: Ord
    {
        if self.root_node.elements.is_empty() {
            return None
        } 

        let removed_value = self.root_node.remove(key);

        // Check if root node is now empty
        if self.root_node.elements.is_empty() & !self.root_node.is_leaf() {
            // Set first child as new root
            self.root_node = self.root_node.children.remove(0);
        }

        removed_value
    }

    pub fn get(&self, key: &K) -> Option<&V>
    where 
        K: Ord    
    {
        self.root_node.search_tree(key)
    }
}

impl<K, V> Default for BTreeMap<K, V> {
    /// Creates a new empty `BTreeMap` with order 6
    fn default() -> Self {
        BTreeMap::new()
    }
}

impl<K, V> Display for BTreeMap<K, V> 
where
    K: Display,
    V: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root_node)
    }
}