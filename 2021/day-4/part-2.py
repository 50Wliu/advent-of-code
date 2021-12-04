# Finds the last instance of bingo and returns the remaining board sum
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
    boardsWithoutBingoes = set(range(len(bingoBoards)))
    for draw in numbersToDraw:
        drawnNumbers.add(draw)
        for boardNum, board in enumerate(bingoBoards):
            if boardNum not in boardsWithoutBingoes:
                continue

            verticalBingo = [True] * 5
            for row in board:
                horizontalBingo = True
                for col, number in enumerate(row):
                    horizontalBingo = horizontalBingo and number in drawnNumbers
                    verticalBingo[col] = verticalBingo[col] and number in drawnNumbers

                if horizontalBingo:
                    boardsWithoutBingoes.remove(boardNum)
                    break

            # Don't remove again if we already had a horizontal bingo
            if True in verticalBingo and boardNum in boardsWithoutBingoes:
                boardsWithoutBingoes.remove(boardNum)

            if len(boardsWithoutBingoes) == 0:
                remainingBoardSum = sum([number for row in board for number in row if number not in drawnNumbers])
                return draw * remainingBoardSum

print(doTheThing())
