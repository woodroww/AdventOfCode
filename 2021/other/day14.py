# https://github.com/womogenes/AoC-2021-Solutions/blob/main/day_14/day_14_p1.py
import string

with open("../day_14_input.txt") as fin:
    raw_data = fin.read().strip().split("\n")

template = raw_data[0]
rules = [line.split(" -> ") for line in raw_data[2:]]

elements = string.ascii_uppercase


def replace(s):
    new_str = ""
    i = 0
    while i < len(s):
        new_str += s[i]
        for start, end in rules:
            if s[i:i + 2] == start:
                new_str += end
                break
        i += 1

    return new_str


for i in range(10):
    template = replace(template)

counts = [template.count(i) for i in elements if template.count(i) != 0]

ans = max(counts) - min(counts)
print(ans)
