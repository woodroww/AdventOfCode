from __future__ import annotations
from typing import Generator

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
#octopi = demo_input

def adjacent(x: int, y: int) -> Generator[tuple[int, int], None, None]:
    for x_d in (-1, 0, 1):
        for y_d in (-1, 0, 1):
            if x_d == y_d == 0:
                continue
            yield x + x_d, y + y_d

coords = {}
for y, line in enumerate(octopi):
    for x, c in enumerate(line):
        coords[x, y] = int(c)

flashes = 0

for _ in range(100):
    todo = []

    for k, v in coords.items():
        coords[k] += 1
        if coords[k] > 9:
            todo.append(k)

    while todo:
        pt = todo.pop()
        if coords[pt] == 0:
            continue
        coords[pt] = 0
        flashes += 1
    
        for other in adjacent(*pt):
            if other in coords and coords[other] != 0:
                coords[other] += 1
                if coords[other] > 9:
                    todo.append(other)

print(f"part 1: {flashes}")
