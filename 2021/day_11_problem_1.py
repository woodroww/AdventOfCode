"""
--- Day 11: Dumbo Octopus ---

You enter a large cavern full of rare bioluminescent dumbo octopuses! They seem
to not like the Christmas lights on your submarine, so you turn them off for
now.

There are 100 octopuses arranged neatly in a 10 by 10 grid. Each octopus slowly
gains energy over time and flashes brightly for a moment when its energy is
full. Although your lights are off, maybe you could navigate through the cave
without disturbing the octopuses if you could predict when the flashes of light
will happen.

Each octopus has an energy level - your submarine can remotely measure the
energy level of each octopus (your puzzle input). For example:

5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526

The energy level of each octopus is a value between 0 and 9. Here, the top-left
octopus has an energy level of 5, the bottom-right one has an energy level of
6, and so on.

You can model the energy levels and flashes of light in steps. During a single
step, the following occurs:

First, the energy level of each octopus increases by 1. Then, any octopus with
an energy level greater than 9 flashes. This increases the energy level of all
adjacent octopuses by 1, including octopuses that are diagonally adjacent. If
this causes an octopus to have an energy level greater than 9, it also flashes.
This process continues as long as new octopuses keep having their energy level
increased beyond 9. (An octopus can only flash at most once per step.) Finally,
any octopus that flashed during this step has its energy level set to 0, as it
used all of its energy to flash. Adjacent flashes can cause an octopus to flash
on a step even if it begins that step with very little energy. Consider the
middle octopus with 1 energy in this situation:

Before any steps:
11111
19991
19191
19991
11111

After step 1:
34543
40004
50005
40004
34543

After step 2:
45654
51115
61116
51115
45654

An octopus is highlighted when it flashed during the given step.

Here is how the larger example above progresses:

Before any steps:
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526

After step 1:
6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637

After step 2:
8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848

After step 3:
0050900866
8500800575
9900000039
9700000041
9935080063
7712300000
7911250009
2211130000
0421125000
0021119000

After step 4:
2263031977
0923031697
0032221150
0041111163
0076191174
0053411122
0042361120
5532241122
1532247211
1132230211

After step 5:
4484144000
2044144000
2253333493
1152333274
1187303285
1164633233
1153472231
6643352233
2643358322
2243341322

After step 6:
5595255111
3155255222
3364444605
2263444496
2298414396
2275744344
2264583342
7754463344
3754469433
3354452433

After step 7:
6707366222
4377366333
4475555827
3496655709
3500625609
3509955566
3486694453
8865585555
4865580644
4465574644

After step 8:
7818477333
5488477444
5697666949
4608766830
4734946730
4740097688
6900007564
0000009666
8000004755
6800007755

After step 9:
9060000644
7800000976
6900000080
5840000082
5858000093
6962400000
8021250009
2221130009
9111128097
7911119976

After step 10:
0481112976
0031112009
0041112504
0081111406
0099111306
0093511233
0442361130
5532252350
0532250600
0032240000
After step 10, there have been a total of 204 flashes. Fast forwarding, here is
the same configuration every 10 steps:

After step 20:
3936556452
5686556806
4496555690
4448655580
4456865570
5680086577
7000009896
0000000344
6000000364
4600009543

After step 30:
0643334118
4253334611
3374333458
2225333337
2229333338
2276733333
2754574565
5544458511
9444447111
7944446119

After step 40:
6211111981
0421111119
0042111115
0003111115
0003111116
0065611111
0532351111
3322234597
2222222976
2222222762

After step 50:
9655556447
4865556805
4486555690
4458655580
4574865570
5700086566
6000009887
8000000533
6800000633
5680000538

After step 60:
2533334200
2743334640
2264333458
2225333337
2225333338
2287833333
3854573455
1854458611
1175447111
1115446111

After step 70:
8211111164
0421111166
0042111114
0004211115
0000211116
0065611111
0532351111
7322235117
5722223475
4572222754

After step 80:
1755555697
5965555609
4486555680
4458655580
4570865570
5700086566
7000008666
0000000990
0000000800
0000000000

After step 90:
7433333522
2643333522
2264333458
2226433337
2222433338
2287833333
2854573333
4854458333
3387779333
3333333333

After step 100:
0397666866
0749766918
0053976933
0004297822
0004229892
0053222877
0532222966
9322228966
7922286866
6789998766

After 100 steps, there have been a total of 1656 flashes.

Given the starting energy levels of the dumbo octopuses in your cavern, simulate 100 steps. How many total flashes are there after 100 steps?

"""

import numpy as np

demo_input = [ 
"5483143223",
"2745854711",
"5264556173",
"6141336146",
"6357385478",
"4167524645",
"2176841721",
"6882881134",
"4846848554",
"5283751526"]

file_name = "day_11_input.txt"
with open(file_name) as f:
    file_input = f.readlines()
real_input = []
for line in file_input:
    real_input.append(line[:-1])

octopi = real_input
octopi = demo_input
lines = []

for octo in octopi:
    string_list = list(octo)
    line = []
    for str_num in string_list:
        line.append(int(str_num))
    lines.append(line)


def adjacent(in_grid, x, y):

    adjacent = {}
    # can go left (-1)
    if x > 0:
        adjacent['up'] = { "energy": in_grid[x-1][y], "x": x-1, "y": y }
    
        if y > 0:
            adjacent['left_up'] = { "energy": in_grid[x-1][y-1], "x": x-1, "y": y-1 }
        if y < in_grid.shape[1] - 1:
            adjacent['right_up'] = { "energy": in_grid[x-1][y+1], "x": x-1, "y": y+1 }

   # can go right (+1)
    if x < in_grid.shape[0] - 1:
        adjacent['down'] = { "energy": in_grid[x+1][y], "x": x+1, "y": y }
        
        if y > 0:
            adjacent["left_down"] = { "energy": in_grid[x+1][y-1], "x": x+1, "y": y-1 }
        if y < in_grid.shape[1] - 1:
            adjacent["right_down"] = { "energy": in_grid[x+1][y+1], "x": x+1, "y": y+1 }

    # can go up (-1)
    if y > 0:
        adjacent['left'] = { "energy": in_grid[x][y-1], "x": x, "y": y-1 }

    # can go down
    if y < in_grid.shape[1] - 1:
        adjacent['right'] = { "energy": in_grid[x][y+1], "x": x, "y": y+1 }
    return adjacent

def flash(in_grid, x, y, flash_grid):

    adj = adjacent(in_grid, x, y)
    
    # the neighboring results of this x,y flash
    for key in adj:
        a = adj[key]
        flash_grid[a["x"]][a["y"]] += 1
    
    current_grid = in_grid + flash_grid
    for y in range(in_grid.shape[1]):
        for x in range(in_grid.shape[0]):
            if current_grid[x][y] > 9:
                flash_grid = flash(in_grid, x, y, flash_grid)

#    print(f"flash: {x},{y}\n{in_grid}")
    return in_grid

def step(in_grid):
    
    in_grid = in_grid + 1
    flash_grid = np.zeros(shape=in_grid.shape, dtype=np.uint16)

    for y in range(in_grid.shape[1]):
        for x in range(in_grid.shape[0]):
            if in_grid[x][y] > 9:
                flash_grid = flash(in_grid, x, y, flash_grid)
    in_grid = in_grid + flash_grid

    #xs, ys = np.where(in_grid > 9)
    in_grid[in_grid > 9] = 0


    print(f"{flash_grid}")
    print(f"{in_grid}")
    return in_grid


grid = np.array(lines)
print(f"step\n{grid}")
grid = step(grid)





    for y in range(in_grid.shape[1]):
        for x in range(in_grid.shape[0]):
            if in_grid[x][y] > 9:
                in_grid[x][y] = 0
    



