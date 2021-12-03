# Return horizontal distance * vertical depth, accounting for aim

from pathlib import Path

horizontal = 0
depth = 0
aim = 0
with open(Path(__file__).with_name("input.txt")) as input:
    for line in input:
        (direction, amount) = line.split()
        amount = int(amount)
        if direction == "forward":
            horizontal += amount
            depth += amount * aim
        elif direction == "down":
            aim += amount
        elif direction == "up":
            aim -= amount
        else:
            raise "Unknown direction!"

print(horizontal * depth)
