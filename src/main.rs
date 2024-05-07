mod tree;
mod node;

use tree::Tree;

fn main() {
    let order = 4;
    let mut tree: Tree<i32> = Tree::new(order);

    tree.insert(5);
    tree.print();
    tree.insert(3);
    tree.print();
    tree.insert(8);
    tree.print();
    tree.insert(9);
    tree.print();
    tree.insert(1);
    tree.print();
    tree.insert(200);
    tree.print();
    tree.insert(100);
    tree.print();
    tree.insert(300);
    tree.print();
    tree.insert(400);
    tree.print();
    tree.insert(500);
    tree.print();
    tree.insert(600);
    tree.print();
    tree.insert(700);
    tree.print();
    tree.insert(800);
    tree.print();
    tree.insert(900);    
    tree.print();
    tree.insert(0);
    tree.print();
}
