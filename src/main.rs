mod tree;
mod node;

use tree::Tree;

fn main() {
    let order = 3;
    let mut tree: Tree<i32, String> = Tree::new(order);

    tree.insert(5, "Hallo".to_string());
    println!("{}", tree);
    tree.insert(10, "Hallo".to_string());
    println!("{}", tree);
    tree.insert(100, "Hallo".to_string());
    println!("{}", tree);
    tree.insert(4, "Hallo".to_string());
    println!("{}", tree);
    tree.insert(0, "Hallo".to_string());
    println!("{}", tree);
    tree.insert(1000, "Hallo".to_string());
    println!("{}", tree);
    tree.insert(2, "Hallo".to_string());
    println!("{}", tree);
    tree.insert(-5, "Hallo".to_string());
    println!("{}", tree);
    
    tree.remove(&5);
    println!("{}", tree);

    tree.remove(&10);
    println!("{}", tree);

    tree.remove(&2);
    println!("{}", tree);

    tree.remove(&1000);
    println!("{}", tree);

    tree.remove(&-5);
    println!("{}", tree);
}
