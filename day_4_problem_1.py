"""
--- Day 4: Giant Squid --- You're already almost 1.5km (almost a mile) below
the surface of the ocean, already so deep that you can't see any sunlight. What
you can see, however, is a giant squid that has attached itself to the outside
of your submarine.

Maybe it wants to play bingo?

Bingo is played on a set of boards each consisting of a 5x5 grid of numbers.
Numbers are chosen at random, and the chosen number is marked on all boards on
which it appears. (Numbers may not appear on all boards.) If all numbers in any
row or any column of a board are marked, that board wins. (Diagonals don't
count.)

The submarine has a bingo subsystem to help passengers (currently, you and the
giant squid) pass the time. It automatically generates a random order
in which to draw numbers and a random set of boards (your puzzle input). For
example:

7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7

After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no
winners, but the boards are marked as follows (shown here adjacent to each
        other to save space):

22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
 8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
 6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
 1 12 20 15 19        14 21 16 12  6         2  0 12  3  7

After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are
still no winners:

22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
 8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
 6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
 1 12 20 15 19        14 21 16 12  6         2  0 12  3  7

Finally, 24 is drawn:

22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
 8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
 6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
 1 12 20 15 19        14 21 16 12  6         2  0 12  3  7

At this point, the third board wins because it has at least one complete row or
column of marked numbers (in this case, the entire top row is marked: 14 21 17
24 4).

The score of the winning board can now be calculated. Start by finding the sum
of all unmarked numbers on that board; in this case, the sum is 188. Then,
multiply that sum by the number that was just called when the board won, 24, to
get the final score, 188 * 24 = 4512.

To guarantee victory against the giant squid, figure out which board will win

first. What will your final score be if you choose that board?

"""

import numpy as np

class BingoCard:
    def __init__(self, lines):
        """
        Input should be a list of 5 strings with 5 numbers
        """
        #print(f"lines:\n{lines}")
        jammy = []
        jimmy = []
        for line in lines:
            # remove any double spaces
            better = " ".join(line.split())
            jammy += better.split(" ")
        for jam in jammy:
            jimmy.append(int(jam))
        
        #print(f"jammy:\n{jammy}")
        self.numbers = np.reshape(np.array(jimmy), (5,5))
        self.marks = np.zeros((5,5), dtype=np.bool)

        #print(self.numbers)
    def mark_ball(self, ball_number):
        #print(f"self.numbers.dtype {self.numbers.dtype}")
        #print(f"type(ball_number) {type(ball_number)}")
        #print(f"self.marks.dtype {self.marks.dtype}")
        ball_int = int(ball_number)
        
        for i in range(5):
            for j in range(5):
#                print(f"{self.numbers[i][j]} == {ball_int}")
                if self.numbers[i][j] == ball_int:
                    self.marks[i][j] = True

    def has_bingo(self):
        for i in range(5):
            if np.count_nonzero(self.marks[i]) > 4:
                return True
            if np.count_nonzero(self.marks.T[i]) > 4:
                return True
        return False
    # the sum of all unmarked numbers
    def sum_unmarked(self):
        total = 0
        for i in range(5):
            for j in range(5):
                if (self.marks[i][j] == False):
                    total += self.numbers[i][j]
        return total

def check_for_winners(cards):
    winners = []
    for card in cards:
        if (card.has_bingo()):
            winners.append(card)
    return winners

def load_data():

    with open("day_4_input.txt") as f:
        lines = f.readlines()

    #print(lines[1:])

    balls = lines.pop(0)[:-1].split(",")
    print(f"balls:\n{balls}")

    cards = []

    while len(lines) > 0:
        line = lines.pop(0)
        if line == "\n":
            card_lines = []
            for i in range(5):
                card_lines.append(lines.pop(0))
            cards.append(BingoCard(card_lines))

    print(f"len(cards) {len(cards)}")
    return balls, cards

def main() -> int:

    balls, cards = load_data()
    ball = balls[0]
    winners = []
    i = 0
    called_balls = []

    while ball and (len(winners) == 0):

        for card in cards:
            card.mark_ball(ball)
        called_balls.append(ball)
        winners = check_for_winners(cards)
        i = i + 1
        if i < len(balls):
            ball = balls[i]
        else:
            ball = None

    # There is one or more winning cards
    if len(winners) > 0:
        for win in winners:
            print(f"Big Winner, Big Winner!!!")
            print(f"Called balls: {called_balls}")
            last_ball_number = int(called_balls[len(called_balls)-1])
            print(f"Last ball: {last_ball_number}")
            sumo = win.sum_unmarked()
            print(f"Sum of unmarked {sumo}")
            print(f"multi jam!! {sumo*last_ball_number}")
            print(f"{win.numbers}")
            print(f"{win.marks}")
    return 0

import sys
if __name__ == '__main__':
    sys.exit(main())
