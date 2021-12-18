# https://github.com/joeyajames/Python/blob/master/graph_adjacency-list.py

class Vertex:

    def __init__(self, n):
        self.name = n
        self.neighbors = list()
    
    def add_neighbor(self, v, weight):
        if v not in self.neighbors:
            self.neighbors.append((v, weight))
            self.neighbors.sort()

class Graph:

    vertices = {}

    def add_vertex(self, vertex):
        if isinstance(vertex, Vertex) and vertex.name not in self.vertices:
            self.vertices[vertex.name] = vertex
            return True
        else:
            return False

    def add_edge(self, u, v, weight=0):
        if u in self.vertices and v in self.vertices:
            self.vertices[u].add_neighbor(v, weight)
            self.vertices[v].add_neighbor(u, weight)
            return True
        else:
            return False

    def print_graph(self):
        for key in sorted(list(self.vertices.keys())):
            print(key + " - " + str(self.vertices[key].neighbors))

    def get_paths(self, start, end, path=[]):
        path = path + [start]

        if start == end:
            return [path]

        if start not in self.vertices:
            return []

        paths = []
        for node_tuple in self.vertices[start].neighbors:
            node = node_tuple[0]
            if node not in path:
                new_paths = self.get_paths(node, end, path)
                for p in new_paths:
                    paths.append(p)

        return paths

# for the advent of code
    def get_all_paths(self, start, end, path=[]):
        path = path + [start]
        print(f"{path}")

        if start == end:
            return [path]

        if start not in self.vertices:
            return []

        paths = []
        
        # start at start and look at its neighbors
        for node_tuple in self.vertices[start].neighbors:
            node = node_tuple[0]
            #weight = node_tuple[1]
            if node not in path:
                new_paths = self.get_all_paths(node, end, path)
                for p in new_paths:
                    paths.append(p)

        return paths












