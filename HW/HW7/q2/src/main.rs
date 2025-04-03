mod tree;

use tree::BinaryTree;

fn main() {
    let mut tree = BinaryTree::new();
    tree.generate_tree(120); // Generate tree with at least 120 nodes
    let diameter = tree.compute_diameter();
    println!("The diameter of the tree is: {}", diameter);
}

#[cfg(test)]
mod tests {
    use super::tree::BinaryTree;

    #[test]
    fn test_diameter() {
        let mut tree = BinaryTree::new();
        // Create a tree matching the example: 0-1, 0-2, 1-3, 1-4
        tree.add_node(None);        // Node 0 (root)
        tree.add_node(Some(0));     // Node 1, child of 0
        tree.add_node(Some(0));     // Node 2, child of 0
        tree.add_node(Some(1));     // Node 3, child of 1
        tree.add_node(Some(1));     // Node 4, child of 1
        let diameter = tree.compute_diameter();
        assert_eq!(diameter, 3);    // Expected diameter is 3
    }
}