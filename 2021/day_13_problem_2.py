"""
--- Day 13: Transparent Origami ---

You reach another volcanically active part of the cave. It would be nice if you
could do some kind of thermal imaging so you could tell ahead of time which
caves are too hot to safely enter.

Fortunately, the submarine seems to be equipped with a thermal camera! When you
activate it, you are greeted with:

Congratulations on your purchase! To activate this infrared thermal imaging
camera system, please enter the code found on page 1 of the manual.

Apparently, the Elves have never used this feature. To your surprise, you
manage to find the manual; as you go to open it, page 1 falls out. It's a large
sheet of transparent paper! The transparent paper is marked with random dots
and includes instructions on how to fold it up (your puzzle input). For
example:

6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5

The first section is a list of dots on the transparent paper. 0,0 represents
the top-left coordinate. The first value, x, increases to the right. The second
value, y, increases downward. So, the coordinate 3,0 is to the right of 0,0,
and the coordinate 0,7 is below 0,0. The coordinates in this example form the
following pattern, where # is a dot on the paper and . is an empty, unmarked
position:

...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
...........
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........

Then, there is a list of fold instructions. Each instruction indicates a line
on the transparent paper and wants you to fold the paper up (for horizontal
y=... lines) or left (for vertical x=... lines). In this example, the
first fold instruction is fold along y=7, which designates the line formed by
all of the positions where y is 7 (marked here with -):

...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
-----------
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........

Because this is a horizontal line, fold the bottom half up. Some of the dots
might end up overlapping after the fold is complete, but dots will never appear
exactly on a fold line. The result of doing this fold looks like this:

#.##..#..#.
#...#......
......#...#
#...#......
.#.#..#.###
...........
...........

Now, only 17 dots are visible.

Notice, for example, the two dots in the bottom left corner before the
transparent paper is folded; after the fold is complete, those dots appear in
the top left corner (at 0,0 and 0,1). Because the paper is transparent, the dot
just below them in the result (at 0,3) remains visible, as it can be seen
through the transparent paper.

Also notice that some dots can end up overlapping; in this case, the dots merge
together and become a single dot.

The second fold instruction is fold along x=5, which indicates this line:

#.##.|#..#.
#...#|.....
.....|#...#
#...#|.....
.#.#.|#.###
.....|.....
.....|.....

Because this is a vertical line, fold left:

#####
#...#
#...#
#...#
#####
.....
.....

The instructions made a square!

The transparent paper is pretty big, so for now, focus on just completing the
first fold. After the first fold in the example above, 17 dots are visible -
dots that end up overlapping after the fold is completed count as a single dot.

How many dots are visible after completing just the first fold instruction on
your transparent paper?

"""

import numpy as np

demo_input = [
'6,10',
'0,14',
'9,10',
'0,3',
'10,4',
'4,11',
'6,0',
'6,12',
'4,1',
'0,13',
'10,12',
'3,4',
'3,0',
'8,4',
'1,10',
'2,14',
'8,10',
'9,0',
'',
'fold along y=7',
'fold along x=5'
]

def print_grid(in_grid):
    for x in range(in_grid.shape[0]):
        for y in range(in_grid.shape[1]):
            if in_grid[x][y] > 0:
                print("#", end="")
            else:
                print(".", end="")
        print()

def horizontal_fold(in_grid, index):
# the line is horizontal and paper folds up
# 'fold along y=7' occurs at grid[7][:] and eliminates that line
# so the top part stays the same
    top = in_grid[:index]
# then the bottom has to be flipped vertically
# and added to the top
    bottom = in_grid[index+1:]
    bottom_flip = np.flipud(bottom)
    result = top + bottom_flip
    return result

def vertical_fold(in_grid, index):
# 'fold along x=5' occurs at grid[:][5] and eliminates that line
# keep left side and fold right over it
    left = in_grid[:,:index]
    #print(f"left")
    #print_grid(left)
    right = in_grid[:,index+1:]
    #print(f"right")
    #print_grid(right)
    right_flip = np.fliplr(right)
    result = left + right_flip
    return result



file_name = "day_13_input.txt"
with open(file_name) as f:
    file_input = f.readlines()
real_input = []
for line in file_input:
    real_input.append(line[:-1])

string_data = real_input
#string_data = demo_input 
blank = 0

for i in range(len(string_data)):
    if string_data[i] == '':
        blank = i
        break;

coordinates = string_data[:blank]
fold_strings = string_data[blank+1:]

numbers_x = []
numbers_y = []

for coord in coordinates:
    x_y = coord.split(",")
    numbers_x.append(int(x_y[1]))
    numbers_y.append(int(x_y[0]))

numbers_x = np.array(numbers_x)
numbers_y = np.array(numbers_y)

grid = np.zeros(shape=(numbers_x.max() + 1, numbers_y.max() + 1), dtype=np.uint16)

for x, y in zip(numbers_x, numbers_y):
        grid[x][y] = 1

folds = []
for fold in fold_strings:
    splitsies = fold.split("=")
    index = splitsies[1]
    axis = splitsies[0][-1]
    folds.append((axis, int(index)))

def printDemo():
    print_grid(grid)
    print()
    work = horizontal_fold(grid, 7)
    print_grid(work)
    print()
    work2 = vertical_fold(work, 5)
    print_grid(work2)
    print()

working_grid = grid

for fold in folds:
    direction = fold[0]
    index = fold[1]
    if direction == 'x':
        working_grid = vertical_fold(working_grid, index)
    elif direction == 'y':
        working_grid = horizontal_fold(working_grid, index)
    else:
        print("something seriously wrong")


print_grid(working_grid)

"""
--- Part Two ---
Finish folding the transparent paper according to the instructions. The manual
says the code is always eight capital letters.

What code do you use to activate the infrared thermal imaging camera system?
"""







