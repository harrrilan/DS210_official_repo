mod pagerank;

use pagerank::{Graph, simulate_walks};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read the graph from data.txt
    let file = File::open("pagerank_data.txt").expect("Cannot open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // First line is the number of vertices
    let n: usize = lines
        .next()
        .expect("File is empty")
        .expect("IO error")
        .trim()
        .parse()
        .expect("Invalid number of vertices");
    let mut graph = Graph::new(n);

    // Read edges
    for line in lines {
        let line = line.expect("IO error");
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let from: usize = parts[0].parse().expect("Invalid vertex label");
            let to: usize = parts[1].parse().expect("Invalid vertex label");
            graph.add_edge(from, to);
        }
    }

    // Simulate walks: 90 walks per vertex, each with 90 steps
    let counts = simulate_walks(&graph, 90, 90);

    // Compute total number of walks
    let total_walks = 90.0 * (graph.n as f64);

    // Prepare vertices for sorting
    let mut vertices: Vec<usize> = (0..graph.n).collect();
    // Sort by termination counts in descending order
    vertices.sort_by(|&a, &b| counts[b].cmp(&counts[a]));

    // Output the top k vertices (k = min(5, n))
    let k = if graph.n < 5 { graph.n } else { 5 };
    for &v in vertices.iter().take(k) {
        let pr = counts[v] as f64 / total_walks;
        println!("vertex {}: approximate PageRank {:.3}", v, pr);
    }
}

/// Test function to verify the implementation on a small graph
#[test]
fn test_pagerank_approximation() {
    // Sample graph: 5 vertices, edges: 0->3, 1->3, 2->3, 3->4, 4->3, 4->0
    let mut graph = Graph::new(5);
    graph.add_edge(0, 3);
    graph.add_edge(1, 3);
    graph.add_edge(2, 3);
    graph.add_edge(3, 4);
    graph.add_edge(4, 3);
    graph.add_edge(4, 0);

    // Simulate walks (using smaller numbers for testing: 10 walks, 10 steps)
    let counts = simulate_walks(&graph, 10, 10);
    let total_walks = 10.0 * 5.0; // 10 walks * 5 vertices
    let pageranks: Vec<f64> = counts.iter().map(|&c| c as f64 / total_walks).collect();

    // Check that the sum of PageRank values is approximately 1
    let sum: f64 = pageranks.iter().sum();
    assert!((sum - 1.0).abs() < 1e-10, "Sum of PageRank should be 1, got {}", sum);

    // Check that vertex 3 has a significant PageRank (since it has many incoming edges)
    let pr_3 = pageranks[3];
    assert!(
        pr_3 > pageranks[1] && pr_3 > pageranks[2],
        "Vertex 3 should have higher PageRank than 1 and 2"
    );
}