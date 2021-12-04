# Finds the first instance of bingo and returns the remaining board sum
# multiplied by the most-recently drawn number.

from pathlib import Path

lines = []
numbersToDraw = []
with open(Path(__file__).with_name("input.txt")) as input:
    numbersToDraw = [int(number) for number in input.readline().strip().split(",")]

    lines = input.readlines()

# Initialize boards
bingoBoards = []
for pos, line in enumerate(lines):
    if len(line.strip()) == 0:
        bingoBoards.append([])
        continue

    numbers = [int(number) for number in line.strip().split()]
    bingoBoards[pos // 6].append(numbers)

def doTheThing():
    drawnNumbers = set()
    for draw in numbersToDraw:
        drawnNumbers.add(draw)
        for board in bingoBoards:
            verticalBingo = [True] * 5
            bingo = False
            for row in board:
                horizontalBingo = True
                for col, number in enumerate(row):
                    horizontalBingo = horizontalBingo and number in drawnNumbers
                    verticalBingo[col] = verticalBingo[col] and number in drawnNumbers

                if horizontalBingo:
                    bingo = True
                    break

            if True in verticalBingo:
                bingo = True

            if bingo:
                remainingBoardSum = sum([number for row in board for number in row if number not in drawnNumbers])
                return draw * remainingBoardSum

print(doTheThing())
