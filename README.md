# B-Tree Map
A sorted Map using a B-Tree to store the key-value pairs.
See https://en.wikipedia.org/wiki/B-tree

By default, it uses a B value of 6, but a custom value can be configured upon creation.

# Usage
Create a new map by using ``new()``
```
use b_tree::BTreeMap;

let mut b_tree = BTreeMap::new()
```

Insert key-value pairs using ``insert(key, value)``
```
b_tree.insert(5, "Five");
```

Retrieve values using ``get(&key)``
```
let value = b_tree.get(&5).unwrap();
```

Remove values using ``remove(&key)``
```
let removed_value = b_tree.remove(&5).unwrap();
```

Configure a custom B value using ``with_order(B)``
(It cant be less lower than 3)
```
let mut custom_b = BTreeMap::with_order(10).unwrap();
```
