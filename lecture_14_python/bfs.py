from collections import deque
from graph import Graph  # Import the Graph class

def compute_and_print_distance_bfs(start, graph):

    # For a graph with n=5 and start=2, here's how it would look:

    distance = [None] * graph.n    # Creates: [None, None, None, None, None]
    distance[start] = 2           # Changes to: [None, None, 0, None, None]
    queue = deque()              # Creates empty queue: deque([])
    queue.append(start)          # Updates queue to: deque([2])
    
    while queue:
        v = queue.popleft()  # Remove first vertex in queue
        for u in graph.outedges[v]:
            if distance[u] is None:  # Consider all unprocessed neighbors of v
                distance[u] = distance[v] + 1
                queue.append(u)

    print("vertex:distance")
    for v in range(graph.n):
        print(f"   {v}:{distance[v]}")
    print()

# Define graph
n = 10
edges = [(0,1),(0,2),(1,2),(2,4),(2,3),(4,3),(4,5),(5,6),(4,6),(6,8),(6,7),(8,7),(1,9)]
edges.sort()
graph = Graph.create_undirected(n, edges)

for i, l in enumerate(graph.outedges):
    print(f"{i} {l}")

# Run BFS from each node
for i in range(graph.n):
    print(f"Distances from node {i}")
    compute_and_print_distance_bfs(i, graph)
