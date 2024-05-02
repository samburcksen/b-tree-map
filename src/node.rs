pub enum Node<T: PartialEq + Eq + PartialOrd + Ord, U> {
    Internal(Internal<T, U>),
    Leaf(Leaf<T, U>),
}

pub struct Root<T: PartialEq + Eq + PartialOrd + Ord, U> {
    key: Option<T>,
    value: Option<U>,
}

impl<T: PartialEq + Eq + PartialOrd + Ord, U> Root<T, U> {
    pub fn new() -> Self {
        Self { 
            key: None, 
            value: None,
        }
    }
}

pub struct Internal<T: PartialEq + Eq + PartialOrd + Ord, U> {
    key: T,
    value: U,
}

impl<T: PartialEq + Eq + PartialOrd + Ord, U> Internal<T, U> {
    pub fn new(key: T, value: U) -> Self {
        Self {
            key,
            value,
        }
    }
}

pub struct Leaf<T: PartialEq + Eq + PartialOrd + Ord, U> {
    key: T,
    value: U,
}

impl<T: PartialEq + Eq + PartialOrd + Ord, U> Leaf<T, U> {
    pub fn new(key: T, value: U) -> Self {
        Self {
            key,
            value,
        }
    }
}
