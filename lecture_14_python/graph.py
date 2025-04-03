from collections import deque

class Graph:
    def __init__(self, n):
        self.n = n
        self.outedges = [[] for _ in range(n)]
    
    def add_directed_edges(self, edges):
        for u, v in edges:
            self.outedges[u].append(v)
    
    def sort_graph_lists(self):
        for l in self.outedges:
            l.sort()
    
    @staticmethod
    def reverse_edges(edges):
        return [(v, u) for u, v in edges]
    
    @classmethod
    def create_directed(cls, n, edges):
        g = cls(n)
        g.add_directed_edges(edges)
        g.sort_graph_lists()
        return g
    
    @classmethod
    def create_undirected(cls, n, edges):
        g = cls.create_directed(n, edges)
        g.add_directed_edges(cls.reverse_edges(edges))
        g.sort_graph_lists()
        return g
