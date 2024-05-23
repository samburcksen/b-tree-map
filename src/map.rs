use std::fmt::Display;

use crate::node::Node;

const DEFAULT_B: usize = 6;

/// A Map using a [B-Tree] to store the key-value pairs.
///
/// B-Trees follow a set of specific rules that need to be
/// considered when inserting and removing elements from it's nodes:
/// * A B-Tree has a fixed B value, that determines the minimum
/// number of children a node has
/// * A node has a maximum of B*2 children
/// * Every node has exactly one key less then it has children
/// * Leafs of a tree have no children
/// * The root node can have less then B children but not less than 2
/// * The keys are ordered from smallest to biggest
///
/// By default, B is set to 6, but a cusomzied B can be
/// specified upon creation.
///
/// The key used for this `BTreeMap` need to implement the [`Ord`] trait.
///
/// [B-Tree]: https://en.wikipedia.org/wiki/B-tree
///
/// # Examples
///
/// ```
/// use b-tree::BTreeMap;
///
/// let mut user_names = BTreeMap::new();
///    
/// // Add users
/// user_names.insert(1, "John");
/// user_names.insert(10, "Steve");
/// user_names.insert(5, "Stefanie");
///    
/// // Remove users
/// assert_eq!(user_names.remove(&5), Some("Stefanie"));
/// assert_eq!(user_names.remove(&2), None);
///
/// // Get users
/// assert_eq!(user_names.get(&1), Some(&"John"));
/// assert_eq!(user_names.get(&2), None);
/// ```
pub struct BTreeMap<K, V> {
    pub(super) min_childs: usize,
    pub(super) root_node: Node<K, V>,
}

impl<K, V> BTreeMap<K, V> {
    /// Creates an empty `BTreeMap` with the dafault
    /// order 6.
    pub fn new() -> Self {
        Self::with_order(DEFAULT_B).unwrap()
    }

    /// Creates a new empty `BTreeMap` with a custom order.
    ///
    /// The order should be greated than 2, otherwise an
    /// Error is returned.
    pub fn with_order(b: usize) -> Result<Self, &'static str> {
        if b < 2 {
            return Err("min_childs needs to be larger than 2!");
        }

        Ok(Self {
            min_childs: b,
            root_node: Node::new(b),
        })
    }

    /// Inserts a key-value pair into the `BTreeMap`.
    ///
    /// If the key is not present in the Map, returns `None`.
    ///
    /// If the key already exists, the value is overriden
    /// and the previous value is returned.
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord + Clone + Default,
        V: Clone + Default,
    {
        if self.root_node.is_full() {
            self.split_root();
        }

        self.root_node.insert(key, value)
    }

    /// Removes a key-value pair from the `BTreeMap` and.
    /// returns the value of the removed pair.
    ///
    /// If the key is not present in the Map, returns `None`.
    pub fn remove(&mut self, key: &K) -> Option<V>
    where
        K: Ord,
    {
        if self.root_node.elements.is_empty() {
            return None;
        }

        let removed_value = self.root_node.remove(key);

        if self.root_node.elements.is_empty() & !self.root_node.is_leaf() {
            // If root is now empty, set first child as new root
            self.root_node = self.root_node.children.remove(0);
        }

        removed_value
    }

    /// Rerurns a reference to a value for the given key.
    ///
    /// If the key is not present in the Map, returns `None`.
    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: Ord,
    {
        self.root_node.search_tree(key)
    }
}

impl<K, V> Default for BTreeMap<K, V> {
    /// Creates an empty `BTreeMap` with order 6
    fn default() -> Self {
        BTreeMap::new()
    }
}

impl<K, V> Display for BTreeMap<K, V>
where
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root_node)
    }
}
