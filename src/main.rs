mod tree;
mod node;

use tree::Tree;

fn main() {
    let order = 4;
    let tree: Tree<i32, String> = Tree::new(order);
}
