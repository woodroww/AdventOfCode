"""
--- Day 1: Report Repair ---
After saving Christmas five years in a row, you've decided to take a vacation
at a nice resort on a tropical island. Surely, Christmas will go on without
you.

The tropical island has its own currency and is entirely cash-only. The gold
coins used there have a little picture of a starfish; the locals just call them
stars. None of the currency exchanges seem to have heard of them, but somehow,
you'll need to find fifty of these coins by the time you arrive so you can pay
the deposit on your room.

To save your vacation, you need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each
day in the Advent calendar; the second puzzle is unlocked when you complete the
first. Each puzzle grants one star. Good luck!

Before you leave, the Elves in accounting just need you to fix your expense
report (your puzzle input); apparently, something isn't quite adding up.

Specifically, they need you to find the two entries that sum to 2020 and then
multiply those two numbers together.

For example, suppose your expense report contained the following:

1721
979
366
299
675
1456

In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying
them together produces 1721 * 299 = 514579, so the correct answer is 514579.

Of course, your expense report is much larger. Find the two entries that sum to
2020; what do you get if you multiply them together?

--- Part Two ---

The Elves in accounting are thankful for your help; one of them even offers you
a starfish coin they had left over from a past vacation. They offer you a
second one if you can find three numbers in your expense report that meet the
same criteria.

Using the above example again, the three entries that sum to 2020 are 979, 366,
and 675. Multiplying them together produces the answer, 241861950.

In your expense report, what is the product of the three entries that sum to
2020?


"""

input_file = "day_1_input.txt"
numbers = []

with open(input_file) as f:
    input_lines = f.readlines()

input_test = \
"""1721
979
366
299
675
1456"""

# for testing
#input_lines = input_test.split("\n")


for line in input_lines:
    numbers.append(int(line))

print(numbers)

winner_1 = 0
winner_2 = 0
winner_3 = 0

for num in numbers:
    other_numbers = numbers.copy()
    other_numbers.remove(num)
    for num2 in other_numbers:
        third_numbers = other_numbers.copy()
        third_numbers.remove(num2)
        for num3 in third_numbers:
            if (num + num2 + num3) == 2020:
                winner_1 = num
                winner_2 = num2
                winner_3 = num3
                break
        if winner_1 != 0:
            break
    if winner_2 != 0:
        break
    

print(f"Winners: {winner_1} {winner_2} {winner_3}")
print(f"Muliplier: {winner_1*winner_2*winner_3}")

# 979, 366, and 675
# Multiplying them together produces the answer, 241861950




