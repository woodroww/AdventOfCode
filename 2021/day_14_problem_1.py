# https://www.youtube.com/watch?v=TIL1JwLtIzw&t=8s

"""
--- Day 14: Extended Polymerization ---

The incredible pressures at this depth are starting to put a strain on your
submarine. The submarine has polymerization equipment that would produce
suitable materials to reinforce the submarine, and the nearby
volcanically-active caves should even have the necessary input elements in
sufficient quantities.

The submarine manual contains instructions for finding the optimal polymer
formula; specifically, it offers a polymer template and a list of pair
insertion rules (your puzzle input). You just need to work out what polymer
would result after repeating the pair insertion process a few times.

For example:

NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C

The first line is the polymer template - this is the starting point of the
process.

The following section defines the pair insertion rules. A rule like AB -> C
means that when elements A and B are immediately adjacent, element C should be
inserted between them. These insertions all happen simultaneously.

So, starting with the polymer template NNCB, the first step simultaneously
considers all three pairs:

The first pair (NN) matches the rule NN -> C, so element C is inserted between
the first N and the second N.
The second pair (NC) matches the rule NC -> B, so element B is inserted between
the N and the C.
The third pair (CB) matches the rule CB -> H, so element H is inserted between
the C and the B.

Note that these pairs overlap: the second element of one pair is the first
element of the next pair. Also, because all pairs are considered
simultaneously, inserted elements are not considered to be part of a pair until
the next step.

After the first step of this process, the polymer becomes NCNBCHB.

Here are the results of a few steps using the above rules:

Template:     NNCB
After step 1: NCNBCHB
After step 2: NBCCNBBBCBHCB
After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB

This polymer grows quickly. After step 5, it has length 97; After step 10, it
has length 3073. After step 10, B occurs 1749 times, C occurs 298 times, H
occurs 161 times, and N occurs 865 times; taking the quantity of the most
common element (B, 1749) and subtracting the quantity of the least common
element (H, 161) produces 1749 - 161 = 1588.

Apply 10 steps of pair insertion to the polymer template and find the most and
least common elements in the result. What do you get if you take the quantity
of the most common element and subtract the quantity of the least common
element?
"""

from collections import Counter

demo_input = [
'NNCB',
'',
'CH -> B',
'HH -> N',
'CB -> H',
'NH -> C',
'HB -> C',
'HC -> B',
'HN -> C',
'NN -> C',
'BH -> H',
'NC -> B',
'NB -> B',
'BN -> B',
'BB -> N',
'BC -> B',
'CC -> N',
'CN -> C'
]

file_name = "day_14_input.txt"
with open(file_name) as f:
    file_input = f.readlines()
real_input = []
for line in file_input:
    real_input.append(line[:-1])

data = real_input
#data = demo_input

pairs = {}

for line in data[2:]:
    pair, insertion = line.split(" -> ")
    pairs[pair] = insertion

# s for sequence
s = data[0]
counter = Counter()
# initial count
for i in range(0, len(s) - 1):
    counter[s[i:i+2]] += 1
    print(s[i:i+2], end=" - ")
    print(f"{s[i]}{pairs[s[i:i+2]]}", end=" ")
    print(f"{pairs[s[i:i+2]]}{s[i]}")

# for final count
char_counts = Counter()
# steps for problem
for step in range(10):
    new_count = Counter()
    for k, v in counter.items():
        #print(f"{k[0]}{pairs[k]}")
        #print(f"{pairs[k]}{k[0]}")
        new_count[f"{k[0]}{pairs[k]}"] += v
        new_count[f"{pairs[k]}{k[1]}"] += v
    
        if step == 9:
            char_counts[k[0]] += v
            char_counts[pairs[k]] += v

    counter = new_count

# due to the way the chars were counted we must add in the last char one time
char_counts[s[-1]] += 1
# to find max and min
values = char_counts.values()
# then aoc wants max - min
print(f"polymer {max(values) - min(values)}")




