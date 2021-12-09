# https://github.com/niristius/Advent-of-Code-2021/blob/main/day6/part1and2.py

import numpy as np
input = open('day_6_input.txt', 'r').read().split(',')

# 80 For part 1, 256 for part 2 - everything else is the same
daysToSimulate = 256

# Crude way to initialize my starting list - don't know of a better way
values, counts = np.unique(input, return_counts=True)
FishMetrics = [[0, 0]]
for i in range(len(values)):
    CurrentValue = []
    CurrentValue.append(int(values[i]))
    CurrentValue.append(int(counts[i]))
    FishMetrics.append(CurrentValue)
for i in range(6, 9):
    CurrentValue = []
    CurrentValue.append(i)
    CurrentValue.append(0)
    FishMetrics.append(CurrentValue)

# The actual simulation
for day in range (0, daysToSimulate):
    Fishamount = 0
    FishesInLabour=FishMetrics[0][1]
    for age in range(1, len(FishMetrics)):
        FishMetrics[age - 1][1] = FishMetrics[age][1]
    FishMetrics[8][1] = FishesInLabour
    FishMetrics[6][1] = FishMetrics[6][1] + FishesInLabour
    for i in range(len(FishMetrics)):
        Fishamount = Fishamount + FishMetrics[i][1]
    print("Day " + str(day+1) + ": " + str(Fishamount) + " fishes")


