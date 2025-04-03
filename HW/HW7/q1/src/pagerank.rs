use rand::Rng;

/// Represents a directed graph with adjacency lists.
pub struct Graph {
    pub n: usize,              // Number of vertices
    pub outgoing: Vec<Vec<usize>>, // Adjacency list: outgoing[i] is the list of vertices reachable from i
}

impl Graph {

    pub fn new(n: usize) -> Self {
        Graph {
            n,
            outgoing: vec![vec![]; n],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.outgoing[from].push(to);
    }
}

pub fn simulate_walks(graph: &Graph, num_walks: usize, num_steps: usize) -> Vec<usize> {
    let mut counts = vec![0; graph.n]; // Empty vector with n zeros
    let mut rng = rand::thread_rng();

    // For each starting vertex
    for v in 0..graph.n {
        // Perform num_walks walks from v
        for _ in 0..num_walks {
            let mut current = v;
            // Each walk consists of num_steps steps
            for _ in 0..num_steps {
                current = step(graph, &mut rng, current);
            }
            // Record where the walk terminates
            counts[current] += 1;
        }
    }
    counts
}

fn step(graph: &Graph, rng: &mut impl Rng, current: usize) -> usize {
    if graph.outgoing[current].is_empty() {
        // No outgoing edges: jump to a random vertex
        rng.gen_range(0..graph.n)
    } else {
        // Has outgoing edges
        if rng.gen::<f64>() < 0.8 {
            // With probability 0.8, follow a random outgoing edge
            let idx = rng.gen_range(0..graph.outgoing[current].len());
            graph.outgoing[current][idx]
        } else {
            // With probability 0.2, jump to a random vertex
            rng.gen_range(0..graph.n)
        }
    }
}