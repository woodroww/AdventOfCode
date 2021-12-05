"""
--- Day 2: Password Philosophy ---
Your flight departs in a few days from the coastal airport; the easiest way
down to the coast from here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day.
"Something's wrong with our computers; we can't log in!" You ask if you can
take a look.

Their password database seems to be a little corrupted: some of the passwords
wouldn't have been allowed by the Official Toboggan Corporate Policy that was
in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of
passwords (according to the corrupted database) and the corporate policy when
that password was set.

For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc

Each line gives the password policy and then the password. The password policy
indicates the lowest and highest number of times a given letter must appear for
the password to be valid. For example, 1-3 a means that the password must
contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is
not; it contains no instances of b, but needs at least 1. The first and third
passwords are valid: they contain one a or nine c, both within the limits of
their respective policies.

How many passwords are valid according to their policies?
"""

input_file = "day_2_input.txt"
with open(input_file) as f:
    in_lines = f.readlines()

demo_lines = \
"""1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"""
demo_lines = demo_lines.split("\n")

# for testing
# lines = demo_lines
lines = in_lines

valid_password_count = 0

def is_valid(in_min, in_max, in_letter, password):
    letter_count = 0
    for letter in password:
        if letter == in_letter:
            letter_count += 1
    if (letter_count > in_max) or (letter_count < in_min):
        return False
    return True


valid_count = 0

for line in lines:
    parts = line.split(": ")
    password = parts[1]
    more_parts = parts[0].split(" ")
    range_string = more_parts[0]
    min_range = int(range_string.split("-")[0])
    max_range = int(range_string.split("-")[1])
    letter = more_parts[1]
    
    isValid = is_valid(min_range, max_range, letter, password)
    if isValid:
        valid_count += 1
    #print(min_range, max_range, letter, password, isValid)
print(f"valid_count: {valid_count}") 


