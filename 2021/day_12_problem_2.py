"""
--- Day 12: Passage Pathing ---
With your submarine's subterranean subsystems subsisting suboptimally, the only
way you're getting out of this cave anytime soon is by finding a path yourself.
Not just a path - the only way to know if you've found the best path is to find
all of them.

Fortunately, the sensors are still mostly working, and so you build a rough map
of the remaining caves (your puzzle input). For example:

start-A
start-b
A-c
A-b
b-d
A-end
b-end

This is a list of how all of the caves are connected. You start in the cave
named start, and your destination is the cave named end. An entry like b-d
means that cave b is connected to cave d - that is, you can move between them.

So, the above cave system looks roughly like this:

    start
    /   \
c--A-----b--d
    \   /
     end

Your goal is to find the number of distinct paths that start at start, end at
end, and don't visit small caves more than once. There are two types of caves:
big caves (written in uppercase, like A) and small caves (written in
lowercase, like b). It would be a waste of time to visit any small
cave more than once, but big caves are large enough that it might be worth
visiting them multiple times. So, all paths you find should visit small
caves at most once, and can visit big caves any number of times.

Given these rules, there are 10 paths through this example cave system:

start,A,b,A,c,A,end
start,A,b,A,end
start,A,b,end
start,A,c,A,b,A,end
start,A,c,A,b,end
start,A,c,A,end
start,A,end
start,b,A,c,A,end
start,b,A,end
start,b,end

(Each line in the above list corresponds to a single path; the caves visited by
that path are listed in the order they are visited and separated by
commas.)

Note that in this cave system, cave d is never visited by any path: to do so,
cave b would need to be visited twice (once on the way to cave d and a second
time when returning from cave d), and since cave b is small, this is
not allowed.

Here is a slightly larger example:

dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc

The 19 paths through it are as follows:

start,HN,dc,HN,end
start,HN,dc,HN,kj,HN,end
start,HN,dc,end
start,HN,dc,kj,HN,end
start,HN,end
start,HN,kj,HN,dc,HN,end
start,HN,kj,HN,dc,end
start,HN,kj,HN,end
start,HN,kj,dc,HN,end
start,HN,kj,dc,end
start,dc,HN,end
start,dc,HN,kj,HN,end
start,dc,end
start,dc,kj,HN,end
start,kj,HN,dc,HN,end
start,kj,HN,dc,end
start,kj,HN,end
start,kj,dc,HN,end
start,kj,dc,end

Finally, this even larger example has 226 paths through it:

fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW

How many paths through this cave system are there that visit small caves at
most once?

--- Part Two ---

After reviewing the available paths, you realize you might have time to visit a
single small cave twice. Specifically, big caves can be visited any number of
times, a single small cave can be visited at most twice, and the remaining
small caves can be visited at most once. However, the caves named start and end
can only be visited exactly once each: once you leave the start cave, you may
not return to it, and once you reach the end cave, the path must end
immediately.

Now, the 36 possible paths through the first example above are:

start,A,b,A,b,A,c,A,end
start,A,b,A,b,A,end
start,A,b,A,b,end
start,A,b,A,c,A,b,A,end
start,A,b,A,c,A,b,end
start,A,b,A,c,A,c,A,end
start,A,b,A,c,A,end
start,A,b,A,end
start,A,b,d,b,A,c,A,end
start,A,b,d,b,A,end
start,A,b,d,b,end
start,A,b,end
start,A,c,A,b,A,b,A,end
start,A,c,A,b,A,b,end
start,A,c,A,b,A,c,A,end
start,A,c,A,b,A,end
start,A,c,A,b,d,b,A,end
start,A,c,A,b,d,b,end
start,A,c,A,b,end
start,A,c,A,c,A,b,A,end
start,A,c,A,c,A,b,end
start,A,c,A,c,A,end
start,A,c,A,end
start,A,end
start,b,A,b,A,c,A,end
start,b,A,b,A,end
start,b,A,b,end
start,b,A,c,A,b,A,end
start,b,A,c,A,b,end
start,b,A,c,A,c,A,end
start,b,A,c,A,end
start,b,A,end
start,b,d,b,A,c,A,end
start,b,d,b,A,end
start,b,d,b,end
start,b,end

The slightly larger example above now has 103 paths through it, and the even
larger example now has 3509 paths through it.

Given these new rules, how many paths through this cave system are there?

"""

import numpy as np

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
# find the number of distinct paths that start at start, end at end, and don't
# visit small caves more than once per traversal
    def get_all_paths(self, start, end, path=[]):
        path = path + [start]
        # print(f"{path}")

        if start == end:
            return [path]

        if start not in self.vertices:
            return []

        paths = []
# Specifically, big caves can be visited any number of
# times, a single small cave can be visited at most twice, and the remaining
# small caves can be visited at most once. However, the caves named start and
# end can only be visited exactly once each       
        # start at start and look at its neighbors
        for node_tuple in self.vertices[start].neighbors:
            node = node_tuple[0]
            #weight = node_tuple[1]

            # so if the node is not already in the path
            # this node not in path is the lowercase check
            # because it would be in path if we have visited
            # it and we can only do that once

            if node.isupper() or node not in path:
                new_paths = self.get_all_paths(node, end, path)
                for p in new_paths:
                    paths.append(p)

        return paths

demo_input = [
'start-A',
'start-b',
'A-c',
'A-b',
'b-d',
'A-end',
'b-end'
]

larger_demo = [
'dc-end',
'HN-start',
'start-kj',
'dc-start',
'dc-HN',
'LN-dc',
'HN-end',
'kj-sa',
'kj-HN',
'kj-dc'
]

largest_demo = [
'fs-end',
'he-DX',
'fs-he',
'start-DX',
'pj-DX',
'end-zg',
'zg-sl',
'zg-pj',
'pj-he',
'RW-he',
'fs-DX',
'pj-RW',
'zg-RW',
'start-pj',
'he-WI',
'zg-he',
'pj-fs',
'start-RW'
]

file_name = "day_12_input.txt"
with open(file_name) as f:
    file_input = f.readlines()
real_input = []
for line in file_input:
    real_input.append(line[:-1])

map_data = real_input
# map_data = largest_demo # larger_demo # demo_input

unique_nodes = []
# index into unique_nodes
small_caves = []

for item in map_data:
    for node in item.split("-"):
        if node not in unique_nodes:
            unique_nodes.append(node)
            if node.islower():
                small_caves.append(len(unique_nodes)-1)
    
graph = Graph()
for v in unique_nodes:
    graph.add_vertex(Vertex(v))

for item in map_data:
    splitsies = item.split("-")
    a = splitsies[0]
    b = splitsies[1]
    added_edge = graph.add_edge(a, b)
    if not added_edge:
        print(f"why can't we add this edge, hu?")


paths = graph.get_all_paths("start", "end")
#graph.print_graph()
print(f"The winner is: {len(paths)}")

"""
    start
    /   \
c--A-----b--d
    \   /
     end

start,A,b,A,c,A,end
start,A,b,A,end
start,A,c,A,b,A,end
start,A,c,A,b,end
start,A,c,A,end
start,b,A,c,A,end

start,A,b,end
start,A,end
start,b,A,end
start,b,end
"""


    






