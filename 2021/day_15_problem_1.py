"""

--- Day 15: Chiton ---

You've almost reached the exit of the cave, but the walls are getting closer
together. Your submarine can barely still fit, though; the main problem is that
the walls of the cave are covered in chitons, and it would be best not to bump
any of them.

The cavern is large, but has a very low ceiling, restricting your motion to two
dimensions. The shape of the cavern resembles a square; a quick scan of chiton
density produces a map of risk level throughout the cave (your puzzle input).
For example:

1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581

You start in the top left position, your destination is the bottom right
position, and you cannot move diagonally. The number at each position is its
risk level; to determine the total risk of an entire path, add up the risk
levels of each position you enter (that is, don't count the risk level of your
starting position unless you enter it; leaving it adds no risk to your
total).

Your goal is to find a path with the lowest total risk. In this example, a path
with the lowest total risk is highlighted here:

1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581

The total risk of this path is 40 (the starting position is never entered, so
its risk is not counted).

What is the lowest total risk of any path from the top left to the bottom
right?

"""

import numpy as np
import sys
import heapdict

demo_input = [
'1163751742',
'1381373672',
'2136511328',
'3694931569',
'7463417111',
'1319128137',
'1359912421',
'3125421639',
'1293138521',
'2311944581',
]


file_name = "day_15_input.txt"
with open(file_name) as f:
    file_input = f.readlines()
real_input = []
for line in file_input:
    real_input.append(line[:-1])

grid = real_input
#grid = demo_input

nums = []

for line in grid:
    s = list(line)
    n = []
    for string_number in s:
        n.append(int(string_number))
    nums.append(n)

grid = np.array(nums)


def adjacent(in_grid, x, y):
    adjacent = {}
    # can go left (-1)
    if x > 0:
        adjacent['up'] = { "danger": in_grid[x-1][y], "x": x-1, "y": y }
   # can go right (+1)
    if x < in_grid.shape[0] - 1:
        adjacent['down'] = { "danger": in_grid[x+1][y], "x": x+1, "y": y }
    # can go up (-1)
    if y > 0:
        adjacent['left'] = { "danger": in_grid[x][y-1], "x": x, "y": y-1 }
    # can go down
    if y < in_grid.shape[1] - 1:
        adjacent['right'] = { "danger": in_grid[x][y+1], "x": x, "y": y+1 }
    return adjacent


best_distances = {}
visited = {}
previous = {}
for y in range(grid.shape[0]):
    for x in range(grid.shape[1]):
        best_distances[(x, y)] = sys.maxsize
        visited[(x, y)] = False
        previous[(x, y)] = (-1, -1)

target_cell = (grid.shape[0] - 1, grid.shape[1] - 1)
start_cell = (0, 0)

# put the start in the queue
queue = heapdict.heapdict()
queue[start_cell] = 0
best_distances[start_cell] = 0

while len(queue) > 0:
    node = queue.popitem()
    visited[node[0]] = True
    if best_distances[node[0]] < node[1]:# optimization
        continue
    x = node[0][0]
    y = node[0][1]
    neighbors = adjacent(grid, x, y)
    for k in neighbors:
        n = neighbors[k]
        coords = (n["x"], n["y"])
        if visited[coords]:
            continue
        newDist = best_distances[node[0]] + n["danger"]

        if newDist < best_distances[coords]:
            previous[coords] = node[0]
            best_distances[coords] = newDist
            queue[coords] = newDist
    # check for end/target_cell
    if node == target_cell:
        break;
    #return best_distances

# shortest path
# best_distances, previous
if best_distances[target_cell] == sys.maxsize:
    print("So we have never found the end ERROR!")

path = []

at = target_cell

while at:
    if at == (-1, -1):
        break
    path.append(at)
    at = previous[at]

path.reverse()

total_risk = 0
for coord in path[1:]: # don't count the first one unless you enter it
    total_risk += grid[coord[0]][coord[1]]
print(f"risk {total_risk}")


