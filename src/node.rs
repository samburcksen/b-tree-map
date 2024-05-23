use std::{cmp::Ordering, fmt::Display, usize};

#[derive(Clone)]
pub(crate) struct KeyValuePair<K, V> {
    pub(crate) key: K,
    pub(crate) value: Box<V>,
}

impl<K, V> KeyValuePair<K, V> {
    pub(crate) fn new(key: K, value: V) -> Self {
        Self {
            key,
            value: value.into(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct Node<K, V> {
    pub(crate) min_childs: usize,
    pub(crate) elements: Vec<KeyValuePair<K, V>>,
    pub(crate) children: Vec<Node<K, V>>,
}

impl<K, V> Node<K, V> {
    pub(crate) fn new(min_childs: usize) -> Self {
        Self {
            min_childs,
            elements: Vec::new(),
            children: Vec::new(),
        }
    }

    pub(crate) fn find_pos(&self, key: &K) -> (usize, bool)
    where
        K: Ord,
    {
        for (index, node_key) in self.elements.iter().enumerate() {
            match key.cmp(&node_key.key) {
                Ordering::Less => return (index, false),
                Ordering::Equal => return (index, true),
                Ordering::Greater => continue,
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

impl<K, V> Default for KeyValuePair<K, V>
where
    K: Default,
    V: Default,
{
    fn default() -> Self {
        Self {
            key: Default::default(),
            value: Default::default(),
        }
    }
}

impl<K, V> Display for Node<K, V>
where
    K: Display,
    V: Display,
{
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
