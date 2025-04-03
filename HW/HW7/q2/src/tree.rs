use rand::Rng;
use std::collections::{HashSet, VecDeque};

/// Represents a node in the binary tree.
#[derive(Debug)]
pub struct TreeNode {
    pub id: usize,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl TreeNode {
    pub fn new(id: usize, parent: Option<usize>) -> Self {
        TreeNode {
            id,
            parent,
            children: Vec::new(),
        }
    }
}

/// Represents the binary tree.
pub struct BinaryTree {
    pub nodes: Vec<TreeNode>,
}

impl BinaryTree {
    /// Creates an empty binary tree.
    pub fn new() -> Self {
        BinaryTree { nodes: Vec::new() }
    }

    /// Adds a new node to the tree, optionally linking it to a parent.
    pub fn add_node(&mut self, parent_id: Option<usize>) -> usize {
        let id = self.nodes.len();
        let node = TreeNode::new(id, parent_id);
        self.nodes.push(node);
        if let Some(parent) = parent_id {
            self.nodes[parent].children.push(id);
        }
        id
    }

    /// Generates a binary tree with at least `min_nodes` nodes.
    /// Each node is randomly assigned 1 or 2 children.
    pub fn generate_tree(&mut self, min_nodes: usize) {
        let mut rng = rand::thread_rng();
        let root_id = self.add_node(None); // Root node
        let mut queue = VecDeque::new();
        queue.push_back(root_id);

        while self.nodes.len() < min_nodes {
            if let Some(parent_id) = queue.pop_front() {
                let num_children = rng.gen_range(1..=2); // Randomly choose 1 or 2
                for _ in 0..num_children {
                    let child_id = self.add_node(Some(parent_id));
                    queue.push_back(child_id);
                }
            }
        }
    }

    /// Performs BFS from a starting node in an undirected view of the tree.
    fn bfs(&self, start: usize) -> (usize, usize) {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((start, 0));
        visited.insert(start);
        let mut farthest_node = start;
        let mut max_distance = 0;

        while let Some((node, distance)) = queue.pop_front() {
            if distance > max_distance {
                max_distance = distance;
                farthest_node = node;
            }
            // Visit children
            for &child in &self.nodes[node].children {
                if !visited.contains(&child) {
                    visited.insert(child);
                    queue.push_back((child, distance + 1));
                }
            }
            // Also visit the parent (if any)
            if let Some(parent) = self.nodes[node].parent {
                if !visited.contains(&parent) {
                    visited.insert(parent);
                    queue.push_back((parent, distance + 1));
                }
            }
        }
        (farthest_node, max_distance)
    }

    /// Computes the diameter of the tree using two BFS traversals.
    pub fn compute_diameter(&self) -> usize {
        if self.nodes.is_empty() {
            return 0;
        }
        // First BFS: Find one endpoint of the diameter
        let (farthest, _) = self.bfs(0);
        // Second BFS: Find the distance to the other endpoint
        let (_, diameter) = self.bfs(farthest);
        diameter
    }
}
