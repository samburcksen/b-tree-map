mod tree;
mod node;

use tree::Tree;

fn main() {
    let order = 4;
    let mut tree: Tree<i32> = Tree::new(order);

    tree.insert(5);
    println!("{}", tree);
    tree.insert(3);
    println!("{tree}");
    tree.insert(8);
    println!("{tree}");
    tree.insert(9);
    println!("{tree}");
    tree.insert(1);
    println!("{tree}");
    tree.insert(200);
    println!("{tree}");
    tree.insert(100);
    println!("{tree}");
    tree.insert(300);
    println!("{tree}");
    tree.insert(400);
    println!("{tree}");
    tree.insert(500);
    println!("{tree}");
    tree.insert(600);
    println!("{tree}");
    tree.insert(700);
    println!("{tree}");
    tree.insert(800);
    println!("{tree}");
    tree.insert(900);    
    println!("{tree}");
    tree.insert(0);
    println!("{tree}");

    tree.remove(500);
    println!("{tree}");
    tree.remove(900);
    println!("{tree}");
    tree.remove(100);
    println!("{tree}");
    tree.remove(8);
    println!("{tree}");
    tree.remove(3);
    println!("{tree}");

    if tree.search(200).is_none() {
        panic!();
    }

    if tree.search(-1).is_some() {
        panic!();
    }
}
