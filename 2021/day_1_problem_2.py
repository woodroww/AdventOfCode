#import os

file_name = "day_1_input.txt"

with open(file_name) as f:
    file_lines = f.readlines()

distance_list = []

for line in file_lines:
    distance_list.append(int(line[:-1]))

print(distance_list[:7])
print(len(distance_list))

i = 0
count = 0
prev_sum = distance_list[i] + \
    distance_list[i+1] + \
    distance_list[i+2]

while i < len(distance_list)-2:
    big_sum = \
    distance_list[i] + \
    distance_list[i+1] + \
    distance_list[i+2]

    if (i < 3):
        print(distance_list[i], distance_list[i+1], distance_list[i+2])
    if big_sum > prev_sum:
        count += 1
    prev_sum = big_sum
    i += 1


print(f"Count: {count}")




