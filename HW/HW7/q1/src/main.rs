pub struct Graph {
    pub n: usize,
    pub outgoing: Vec<Vec<usize>>,
}

fn adj_list(n:usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut outgoing = vec![vec![]; n];
    
    for &(a, b) in edges {
        outgoing[a].push(b);  // Add each edge
    }
    outgoing
}

