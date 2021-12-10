"""
--- Day 9: Smoke Basin ---

These caves seem to be lava tubes. Parts are even still volcanically active;
small hydrothermal vents release smoke into the caves that slowly settles like
rain.

If you can model how the smoke flows through the caves, you might be able to
avoid it and be that much safer. The submarine generates a heightmap of the
floor of the nearby caves for you (your puzzle input).

Smoke flows to the lowest point of the area it's in. For example, consider the
following heightmap:

2199943210
3987894921
9856789892
8767896789
9899965678

Each number corresponds to the height of a particular location, where 9 is the
highest and 0 is the lowest a location can be.

Your first goal is to find the low points - the locations that are lower than
any of its adjacent locations. Most locations have four adjacent locations (up,
down, left, and right); locations on the edge or corner of the map have
three or two adjacent locations, respectively. (Diagonal locations do not count
as adjacent.)

In the above example, there are four low points, all highlighted: two are in
the first row (a 1 and a 0), one is in the third row (a 5), and one is in the
bottom row (also a 5). All other locations on the heightmap have some lower
adjacent location, and so are not low points.

The risk level of a low point is 1 plus its height. In the above example, the
risk levels of the low points are 2, 1, 6, and 6. The sum of the risk levels of
all low points in the heightmap is therefore 15.

Find all of the low points on your heightmap. What is the sum of the risk
levels of all low points on your heightmap?

--- Part Two ---

Next, you need to find the largest basins so you know what areas are most
important to avoid.

A basin is all locations that eventually flow downward to a single low point.
Therefore, every low point has a basin, although some basins are very small.
Locations of height 9 do not count as being in any basin, and all other
locations will always be part of exactly one basin.

The size of a basin is the number of locations within the basin, including the
low point. The example above has four basins.

The top-left basin, size 3:

2199943210
3987894921
9856789892
8767896789
9899965678

The top-right basin, size 9:

2199943210
3987894921
9856789892
8767896789
9899965678

The middle basin, size 14:

2199943210
3987894921
9856789892
8767896789
9899965678

The bottom-right basin, size 9:

2199943210
3987894921
9856789892
8767896789
9899965678

Find the three largest basins and multiply their sizes together. In the above
example, this is 9 * 14 * 9 = 1134.

What do you get if you multiply together the sizes of the three largest basins?

"""

import numpy as np

demo_input = [ 
'2199943210',
'3987894921',
'9856789892',
'8767896789',
'9899965678'
]

file_name = "day_9_input.txt"
with open(file_name) as f:
    file_input = f.readlines()
real_input = []
for line in file_input:
    real_input.append(line[:-1])

digit_data = real_input
digit_data = demo_input

numbers = []
for line_string in digit_data:
    line_numbers = []
    chars = list(line_string)
    for char in chars:
        line_numbers.append(int(char))
    numbers.append(line_numbers)

grid = np.array(numbers)


def adjacent_point_values(in_grid, x, y):
    adjacent = {}
    # can go left (-1)
    if x > 0:
        adjacent['left'] = in_grid[x-1][y]
        adjacent['left_x'] = x-1
        adjacent['left_y'] = y
    # can go right (+1)
    if x < in_grid.shape[0] - 1:
        adjacent['right'] = in_grid[x+1][y]
        adjacent['right_x'] = x+1
        adjacent['right_y'] = y
    # can go up (-1)
    if y > 0:
        adjacent['up'] = in_grid[x][y-1]
        adjacent['up_x'] = x
        adjacent['up_y'] = y-1
    # can go down
    if y < in_grid.shape[1] - 1:
        adjacent['down'] = in_grid[x][y+1]
        adjacent['down_x'] = x
        adjacent['down_y'] = y+1
    return adjacent

def lowestPoints(in_grid, add_height):
    """
    add_height for risk values
    """
    lowpoints = np.zeros(shape=grid.shape, dtype=np.uint8)
    for y in range(in_grid.shape[1]):
        for x in range(in_grid.shape[0]):
            
            current_height = in_grid[x][y]
            # 10 is invalid on the board
            adjacent = adjacent_point_values(in_grid, x, y)
            
            lowspot = True

            for key in adjacent.keys():
                if (adjacent[key] <= current_height):
                    lowspot = False
            if lowspot:
                if add_height:
                    lowpoints[x][y] = 1 + current_height
                else:
                    lowpoints[x][y] = 1
    return lowpoints


risk_levels = lowestPoints(grid, True)
print(f"sum of risk levels {risk_levels.sum()}")



def findBasin(in_grid, x, y):

    found = []
    current = in_grid[x][y]
    adjacent = adjacent_point_values(in_grid, x, y)
    if (adjacent['up'] > current) and (adjacent['up'] < 9):
        found.append([adjacent['up_x'], adjacent['up_y']])
        found += findBasin(in_grid, x, y)
    if (adjacent['down'] > current) and (adjacent['down'] < 9):
        found.append([adjacent['down_x'], adjacent['down_y']])
        found += findBasin(in_grid, x, y)
    if (adjacent['left'] > current) and (adjacent['left'] < 9):
        found.append([adjacent['left_x'], adjacent['left_y']])
        found += findBasin(in_grid, x, y)
    if (adjacent['right'] > current) and (adjacent['right'] < 9):
        found.append([adjacent['right_x'], adjacent['right_y']])
        found += findBasin(in_grid, x, y)

    return found



low_points = lowestPoints(grid, False)
low_xs, low_ys = np.where(low_points == 1)

for 






