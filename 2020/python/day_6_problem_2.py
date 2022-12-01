"""
--- Day 6: Custom Customs ---

As your flight approaches the regional airport where you'll switch to a much
larger plane, customs declaration forms are distributed to the passengers.

The form asks a series of 26 yes-or-no questions marked a through z. All you
need to do is identify the questions for which anyone in your group answers
"yes". Since your group is just you, this doesn't take very long.

However, the person sitting next to you seems to be experiencing a language
barrier and asks if you can help. For each of the people in their group, you
write down the questions for which they answer "yes", one per line. For
example:

abcx
abcy
abcz

In this group, there are 6 questions to which anyone answered "yes": a, b, c,
x, y, and z. (Duplicate answers to the same question don't count extra; each
        question counts at most once.)

Another group asks for your help, then another, and eventually you've collected
answers from every group on the plane (your puzzle input). Each group's answers
are separated by a blank line, and within each group, each person's answers are
on a single line. For example:

abc

a
b
c

ab
ac

a
a
a
a

b

This list represents answers from five groups:

The first group contains one person who answered "yes" to 3 questions: a, b, and c.
The second group contains three people; combined, they answered "yes" to 3 questions: a, b, and c.
The third group contains two people; combined, they answered "yes" to 3 questions: a, b, and c.
The fourth group contains four people; combined, they answered "yes" to only 1 question, a.
The last group contains one person who answered "yes" to only 1 question, b.
In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.

For each group, count the number of questions to which anyone answered "yes".
What is the sum of those counts?

--- Part Two ---

As you finish the last group's customs declaration, you notice that you misread
one word in the instructions:

You don't need to identify the questions to which anyone answered "yes"; you
need to identify the questions to which everyone answered "yes"!

Using the same example as above:

abc

a
b
c

ab
ac

a
a
a
a

b
This list represents answers from five groups:

In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
In the second group, there is no question to which everyone answered "yes".
In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
In the fourth group, everyone answered yes to only 1 question, a.
In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.
In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.

For each group, count the number of questions to which everyone answered "yes".
What is the sum of those counts?


"""

import numpy as np

file_name = "day_6_input.txt"
with open(file_name) as f:
    file_input = f.readlines()
real_input = []
for line in file_input:
    real_input.append(line[:-1]) # remove newline


demo_input = [
'abc',
'',
'a',
'b',
'c',
'',
'ab',
'ac',
'',
'a',
'a',
'a',
'a',
'',
'b'
]

some_letters = real_input
#some_letters = demo_input

# make groups
groups = []
group = []

for line in some_letters:
    if len(line) > 0:
        group.append(line)
    else:
        groups.append(group)
        group = []
groups.append(group)


group_answers = []
group_people_count = []

for g in groups:
    # g a groups list of answers
    yes_answers = np.zeros(26, dtype=np.uint8)
    num_people = len(g)
    group_people_count.append(num_people)
    for line in g: # this is one persons answers
        for character in line:
            alph_idx = ord(character) - 97 # 0 based alphabet index 
            yes_answers[alph_idx] += 1 # count this now in part 2
    group_answers.append(yes_answers)

def print_infos():
    for i, g in enumerate(groups):
        print(f"orginal:  {g}")
        print(f"answers:  {group_answers[i]}")
        print(f"# people: {group_people_count[i]}")

# For each group, count the number of questions to which everyone answered "yes".
# What is the sum of those counts?

yes_sum = 0
for i, ans in enumerate(group_answers):
    all_yes = np.count_nonzero(ans == group_people_count[i])  
    yes_sum += all_yes 

print(f"Sum: {yes_sum}")


















