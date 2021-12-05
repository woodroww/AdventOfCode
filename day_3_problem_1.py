"""
You need to use the binary numbers in the diagnostic report to generate two new
binary numbers (called the gamma rate and the epsilon rate). The power
consumption can then be found by multiplying the gamma rate by the epsilon
rate.

Each bit in the gamma rate can be determined by finding the most common bit in
the corresponding position of all numbers in the diagnostic report. For
example, given the following diagnostic report:

1st bit, 2nd bit...
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010

Step 1: find most common bit (0 or 1) in the columns, that is the bit of the
gamma rate.
Step 2: use the least common bit for the epsilon
Step 3: convert the binary numbers into decimal numbers
and multiply gamma*epsilon

Use the binary numbers in your diagnostic report to calculate the gamma rate
and epsilon rate, then multiply them together. What is the power consumption of
the submarine? (Be sure to represent your answer in decimal, not binary.)

"""

input_file = "day_3_input.txt"
lines = None
binary_strings = []

with open(input_file) as f:
    lines = f.readlines()

for line in lines:
    binary_strings.append(line[:-1])

print(f"len(lines): {len(lines)}")
print(lines[:3])

one_count = 0
zero_count = 0

print(len(binary_strings))
print(type(binary_strings[0]))
bit_count = len(binary_strings[0])

gamma = ""
epsilon = ""

print(f"bit_count {bit_count} {list(range(bit_count))}")

for char in range(bit_count):
    one_count = 0
    zero_count = 0

    for line in binary_strings:
        if line[char] == '1':
            one_count += 1
        elif line[char] == '0':
            zero_count += 1
        else:
            print("Error")
            break
    if one_count > zero_count:
        gamma += '1'
        epsilon += '0'
    elif one_count < zero_count:
        gamma += '0'
        epsilon += '1'

g = int(gamma, 2)
e = int(epsilon, 2)
print(f"Gamma {gamma} {g}")
print(f"Epsilon {epsilon} {e}")
print(f"Power consumption: {e*g}")











