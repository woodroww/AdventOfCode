"""
--- Day 7: The Treachery of Whales ---

A giant whale has decided your submarine is its next meal, and it's much faster
than you are. There's nowhere to run!

Suddenly, a swarm of crabs (each in its own tiny submarine - it's too deep for
them otherwise) zooms in to rescue you! They seem to be preparing to
blast a hole in the ocean floor; sensors indicate a massive underground cave
system just beyond where they're aiming!

The crab submarines all need to be aligned before they'll have enough power to
blast a large enough hole for your submarine to get through. However, it
doesn't look like they'll be aligned before the whale catches you! Maybe you
can help?

There's one major catch - crab submarines can only move horizontally.

You quickly make a list of the horizontal position of each crab (your puzzle
input). Crab submarines have limited fuel, so you need to find a way to
make all of their horizontal positions match while requiring them to spend as
little fuel as possible.

For example, consider the following horizontal positions:

16,1,2,0,4,2,7,1,2,14

This means there's a crab with horizontal position 16, a crab with horizontal
position 1, and so on.

Each change of 1 step in horizontal position of a single crab costs 1 fuel. You
could choose any horizontal position to align them all on, but the one that
costs the least fuel is horizontal position 2:

Move from 16 to 2: 14 fuel
Move from 1 to 2: 1 fuel
Move from 2 to 2: 0 fuel
Move from 0 to 2: 2 fuel
Move from 4 to 2: 2 fuel
Move from 2 to 2: 0 fuel
Move from 7 to 2: 5 fuel
Move from 1 to 2: 1 fuel
Move from 2 to 2: 0 fuel
Move from 14 to 2: 12 fuel

This costs a total of 37 fuel. This is the cheapest possible outcome; more
expensive outcomes include aligning at position 1 (41 fuel), position 3 (39
fuel), or position 10 (71 fuel).

Determine the horizontal position that the crabs can align to using the least
fuel possible. How much fuel must they spend to align to that position?


"""

import numpy as np

demo_input = np.array([16,1,2,0,4,2,7,1,2,14])

file_name = "day_7_input.txt"
with open(file_name) as f:
    file_input = f.readlines()
real_input = file_input[0][:-1].split(",")
real_input = np.array(real_input, dtype=np.int16)

bam = (real_input.max() - real_input.min()) // 2

import matplotlib.pyplot as plt

plt.hist(real_input)
plt.show()

#unique_elements, counts_elements = np.unique(real_input, return_counts=True)
histy = np.histogram(real_input)

crabs = real_input
#crabs = demo_input

def fuel_spent(k):
    moves = np.abs(crabs - k)
    total_fuel = moves.sum()
    return total_fuel
    
formula = ( (crabs.mean() + .5) - (crabs.mean() - .5) ) / 2
print(f"k = {formula}")
print(fuel_spent(formula))

# How much fuel must they spend to align to that position?

min_fuel = 650000000
ans = 0

spent_fuels = []
for i in range(crabs.max()+1):
    fuel = fuel_spent(i)
    spent_fuels.append(fuel)
position = np.argmin(spent_fuels)
print(f"Position {position}")
print(f"Fuel Spent {spent_fuels[position]}")

# median is also the answer
crabs[len(crabs) // 2]


