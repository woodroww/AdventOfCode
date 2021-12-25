"""
--- Day 17: Trick Shot ---
You finally decode the Elves' message. HI, the message says. You continue
searching for the sleigh keys.

Ahead of you is what appears to be a large ocean trench. Could the keys have
fallen into it? You'd better send a probe to investigate.

The probe launcher on your submarine can fire the probe with any integer
velocity in the x (forward) and y (upward, or downward if negative) directions.
For example, an initial x,y velocity like 0,10 would fire the probe straight
up, while an initial velocity like 10,-1 would fire the probe forward at a
slight downward angle.

The probe's x,y position starts at 0,0. Then, it will follow some trajectory by
moving in steps. On each step, these changes occur in the following order:

The probe's x position increases by its x velocity.
The probe's y position increases by its y velocity.
Due to drag, the probe's x velocity changes by 1 toward the value 0; that is,
it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0,
or does not change if it is already 0.
Due to gravity, the probe's y velocity decreases by 1.
For the probe to successfully make it into the trench, the probe must be on
some trajectory that causes it to be within a target area after any step. The
submarine computer has already calculated this target area (your puzzle input).
For example:

target area: x=20..30, y=-10..-5
This target area means that you need to find initial x,y velocity values such
that after any step, the probe's x position is at least 20 and at most 30, and
the probe's y position is at least -10 and at most -5.

Given this target area, one initial velocity that causes the probe to be within
the target area after any step is 7,2:

.............#....#............
.......#..............#........
...............................
S........................#.....
...............................
...............................
...........................#...
...............................
....................TTTTTTTTTTT
....................TTTTTTTTTTT
....................TTTTTTTT#TT
....................TTTTTTTTTTT
....................TTTTTTTTTTT
....................TTTTTTTTTTT

In this diagram, S is the probe's initial position, 0,0. The x coordinate
increases to the right, and the y coordinate increases upward. In the bottom
right, positions that are within the target area are shown as T. After each
step (until the target area is reached), the position of the probe is marked
with #. (The bottom-right # is both a position the probe reaches and a position
in the target area.)

Another initial velocity that causes the probe to be within the target area
after any step is 6,3: in target at 21,-9

...............#..#............
...........#........#..........
...............................
......#..............#.........
...............................
...............................
S....................#.........
...............................
...............................
...............................
.....................#.........
....................TTTTTTTTTTT
....................TTTTTTTTTTT
....................TTTTTTTTTTT
....................TTTTTTTTTTT
....................T#TTTTTTTTT
....................TTTTTTTTTTT
Another one is 9,0: in target at 30, -6

S........#.....................
.................#.............
...............................
........................#......
...............................
....................TTTTTTTTTTT
....................TTTTTTTTTT#
....................TTTTTTTTTTT
....................TTTTTTTTTTT
....................TTTTTTTTTTT
....................TTTTTTTTTTT
One initial velocity that doesn't cause the probe to be within the target area
after any step is 17,-4:

S..............................................................
...............................................................
...............................................................
...............................................................
.................#.............................................
....................TTTTTTTTTTT................................
....................TTTTTTTTTTT................................
....................TTTTTTTTTTT................................
....................TTTTTTTTTTT................................
....................TTTTTTTTTTT..#.............................
....................TTTTTTTTTTT................................
...............................................................
...............................................................
...............................................................
...............................................................
................................................#..............
...............................................................
...............................................................
...............................................................
...............................................................
...............................................................
...............................................................
..............................................................#
The probe appears to pass through the target area, but is never within it after
any step. Instead, it continues down and to the right - only the first few
steps are shown.

If you're going to fire a highly scientific probe out of a super cool probe
launcher, you might as well do it with style. How high can you make the probe
go while still reaching the target area?

In the above example, using an initial velocity of 6,9 is the best you can do,
causing the probe to reach a maximum y position of 45. (Any higher initial y
velocity causes the probe to overshoot the target area entirely.)

Find the initial velocity that causes the probe to reach the highest y position
and still eventually be within the target area after any step. What is the
highest y position it reaches on this trajectory?
"""



import sys
import numpy as np

demo_input = "target area: x=20..30, y=-10..-5"
 
file_name = "day_17_input.txt"
with open(file_name) as f:
    file_input = f.readlines()
real_input = file_input[0][:-1]

input_data = real_input
input_data = demo_input

comma_split = input_data.split(", ")
target_x_ranges = []
target_y_ranges = []
x_idx = comma_split[0].find("x")
if x_idx != -1:
    target_x_ranges = comma_split[0][x_idx+2:].split("..")
else:
    print(f"We have a major problem")
    exit(1)
target_y_ranges = comma_split[1][2:].split("..")
target_top_left = [int(target_x_ranges[0]), int(target_y_ranges[1])]
target_bottom_right = [int(target_x_ranges[1]), int(target_y_ranges[0])]

class Probe:
    # initial velocities
    def __init__(self, x_velocity, y_velocity):
        self.x = 0
        self.y = 0
        self.x_v = x_velocity
        self.y_v = y_velocity
    def step(self):
        self.x += self.x_v
        self.y += self.y_v
        if self.x_v > 0:
            self.x_v -= 1
        elif self.x_v < 0:
            self.x_v += 1
        self.y_v -= 1
    def in_target(self, top_left, bottom_right):
        if self.x >= top_left[0] and self.x <= bottom_right[0] and\
             self.y <= top_left[1] and self.y >= bottom_right[1]:
               return True
        return False
    def __str__(self):
        return f"x,y: {self.x},{self.y} velocity: {self.x_v},{self.y_v}"
    def __repr__(self):
        return f"x,y: {self.x},{self.y} velocity: {self.x_v},{self.y_v}"

# We want the shot that goes the highest and ends up in target

# step through return True if landed in target on a step
# return false if probe misses target
def fire_probe(probe):

    flying_correctly = True
    steps = [[probe.x, probe.y]]

    while flying_correctly:

        probe.step()
        steps.append([probe.x, probe.y])

        if probe.in_target(target_top_left, target_bottom_right):
            print(f"In target at {probe.x}, {probe.y}")
            return True, steps
        
        # gone past the target
        if probe.x > target_bottom_right[0]:
            flying_correctly = False
            print(f"past target on x")
        # probe has stopped moving in x and has still not reached the target
        elif probe.x_v == 0 and probe.x < target_top_left[0]:
            flying_correctly = False
            print(f"fell short of target on x")
        # probe y is under target
        elif probe.y < target_bottom_right[1]:
            flying_correctly = False
            print(f"probe y is under target")
    
    return False, steps

def point_is_target(x, y, top_left, bottom_right):
    if x >= top_left[0] and x <= bottom_right[0] and\
         y <= top_left[1] and y >= bottom_right[1]:
           return True
    return False

def at_probe_point(in_steps, x, y):
    for xy in in_steps:
        if xy[0] == x and xy[1] == y:
            return True
    return False

def plot_map(in_steps):
    in_steps = np.array(in_steps)
    maxs = np.amax(in_steps, axis=0)
    mins = np.amin(in_steps, axis=0)
    max_x = maxs[0]
    max_y = maxs[1]
    min_x = mins[0]
    min_y = mins[1]
    for y in range(max_y, min_y-1, -1):
        line = ""
        for x in range(min_x, max_x+1):
            if at_probe_point(in_steps, x, y):
                line += "#"
            elif point_is_target(x, y, target_top_left, target_bottom_right):
                line += "T"
            else:
                line += "."
        print(line)

print(f"Target:")
print(f"{target_top_left[0]}, {target_top_left[1]}   {target_bottom_right[0]}, {target_top_left[1]}")
print(f"{target_top_left[0]}, {target_bottom_right[1]}  {target_bottom_right[0]}, {target_bottom_right[1]}")

probe = Probe(7, 2) # in target at 28, -7
in_target, steps = fire_probe(probe)
plot_map(steps)

probe = Probe(6, 3) # in target at 21, -9
in_target, steps = fire_probe(probe)
plot_map(steps)

probe = Probe(9, 0) # in target at 30, -6
in_target, steps = fire_probe(probe)
plot_map(steps)

probe = Probe(17,-4) # miss
in_target, steps = fire_probe(probe)
plot_map(steps)



