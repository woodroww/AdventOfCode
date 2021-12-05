"""
--- Day 5: Hydrothermal Venture ---
You come across a field of hydrothermal vents on the ocean floor! These vents
constantly produce large, opaque clouds, so it would be best to avoid them if
possible.

They tend to form in lines; the submarine helpfully produces a list of nearby
lines of vents (your puzzle input) for you to review. For example:

0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
Each line of vents is given as a line segment in the format x1,y1 -> x2,y2
where x1,y1 are the coordinates of one end the line segment and x2,y2 are the
coordinates of the other end. These line segments include the points at both
ends. In other words:

An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
For now, only consider horizontal and vertical lines:
lines where either x1 = x2 or y1 = y2.

So, the horizontal and vertical lines from the above list would produce the
following diagram:

.......1.. 0
..1....1.. 1
..1....1.. 2
.......1.. 3
.112111211 4
.......... 5
.......... 6
.......... 7
.......... 8
222111.... 9

0123456789

In this diagram, the top left corner is 0,0 and the bottom right corner is 9,9.
Each position is shown as the number of lines which cover that point or . if no
line covers that point. The top-left pair of 1s, for example, comes from 2,2 ->
2,1; the very bottom row is formed by the overlapping lines 0,9 -> 5,9 and 0,9
-> 2,9.

To avoid the most dangerous areas, you need to determine the number of points
where at least two lines overlap. In the above example, this is anywhere in the
diagram with a 2 or larger - a total of 5 points.

Consider only horizontal and vertical lines. At how many points do at least two
lines overlap?
"""

import numpy as np

test_input =\
"""0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"""


with open("day_5_input.txt") as f:
    input_lines = f.readlines()


test_lines = test_input.split("\n")

print(f"test_lines: {test_lines[:5]}")
print(f"input_lines: {input_lines[:5]}")

def format_input(in_lines):
    splitsies = []
    for line in in_lines:
        #print(f"line: {line}")    
        vals = line.split(" -> ")
        #print(f"vals: {vals}")
        xy1 = vals[0].split(",")
        xy2 = vals[1].split(",")
        splitsies.append([int(xy1[0]), int(xy1[1]), int(xy2[0]), int(xy2[1])])
    return np.array(splitsies)

test_split = format_input(test_lines)
print(f"test_split:\n{test_split[:5]}")

real_split = format_input(input_lines)
print(f"real_split:\n{real_split[:5]}")

dobie = """
.......1.. 0
..1....1.. 1
..1....1.. 2
.......1.. 3
.112111211 4
.......... 5
.......... 6
.......... 7
.......... 8
222111.... 9

0123456789
"""
dobie_diagonals = """
1 0 1 0 0 0 0 1 1 0
0 1 1 1 0 0 0 2 0 0
0 0 2 0 1 0 1 1 1 0
0 0 0 1 0 2 0 2 0 0
0 1 1 2 3 1 3 2 1 1
0 0 0 1 0 2 0 0 0 0 
0 0 1 0 0 0 1 0 0 0
0 1 0 0 0 0 0 1 0 0
1 0 0 0 0 0 0 0 1 0
2 2 2 1 1 1 0 0 0 0
"""

# Consider only horizontal and vertical lines. At how many points do at least
# two lines overlap?

print(f"test_split.max() {test_split.max()}")
print(f"test_split.min() {test_split.min()}")
print(f"real_split.max() {real_split.max()}")
print(f"real_split.min() {real_split.min()}")


#print(f"x1\n{x1}")
#print(f"y1\n{y1}")
#print(f"x2\n{x2}")
#print(f"y2\n{y2}")

#For now, only consider horizontal and vertical lines:
# lines where either x1 = x2 or y1 = y2.

#print(test_split[x1 == x2])
#print(test_split[y1 == y2])
def print_point_list(in_list):
    print(f"{in_list[0]},{in_list[1]} -> {in_list[2]},{in_list[3]}")

def make_grid(in_split):

    x1 = in_split[:,0]
    y1 = in_split[:,1]
    x2 = in_split[:,2]
    y2 = in_split[:,3]

    grid = np.zeros((in_split.max()+1, in_split.max()+1), dtype=np.uint16)

#    points = np.concatenate((in_split[x1 == x2], in_split[y1 == y2]), axis=0)
    
    for point_list in in_split:
        #print_point_list(point_list)
        # x1 = [0], y1 = [1], x2 = [2], y2 = [3]
        if point_list[0] == point_list[2]: # x's are the same
            if point_list[1] <= point_list[3]:
                grid[point_list[1]:point_list[3]+1, point_list[0]] += 1
            else:
                grid[point_list[3]:point_list[1]+1, point_list[0]] += 1
     
        elif point_list[1] == point_list[3]: # y's are the same
            if point_list[0] <= point_list[2]:
                grid[point_list[1], point_list[0]:point_list[2]+1] += 1
            else:
                grid[point_list[1], point_list[2]:point_list[0]+1] += 1

        else:
            #print(f"diagonal", end=": ")
            #print_point_list(point_list)
            x_range_dir = 1
            y_range_dir = 1

            if point_list[0] > point_list[2]:
                x_range_dir = -1
                
            x_min = point_list[0]
            x_max = point_list[2]
            y_min = point_list[1]
            y_max = point_list[3]

            if point_list[1] > point_list[3]:
                y_range_dir = -1     
           
            #print(f"x_min:{x_min} x_max:{x_max} x_range_dir:{x_range_dir}")
            #print(f"y_min:{y_min} y_max:{y_max} y_range_dir:{y_range_dir}")
            xs = list(range(x_min, x_max+x_range_dir, x_range_dir))
            ys = list(range(y_min, y_max+y_range_dir, y_range_dir))
            #print(f"xs:{xs}")
            #print(f"ys:{ys}")
            
            for i,j in zip(xs, ys):
                grid[j][i] += 1
    
    print(f"goal:{dobie_diagonals}")
    print(f"grid:\n{grid}")

    zam = np.where(grid.flatten() > 1)
    print(f"zam: {len(zam[0])}")

make_grid(test_split)
make_grid(real_split)


