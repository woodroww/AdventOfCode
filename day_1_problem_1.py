#import os

file_name = "day_1_input.txt"

with open(file_name) as f:
    file_lines = f.readlines()

distance_list = []

for line in file_lines:
    distance_list.append(int(line[:-1]))

print(distance_list[:5])
print(len(distance_list))

previous_distance = distance_list[0]
larger_count = 0

for distance in distance_list:
    if (distance > previous_distance):
        larger_count += 1
    previous_distance = distance

print(f"Count: {larger_count}")




