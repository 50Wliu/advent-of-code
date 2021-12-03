# Return horizontal distance * vertical depth

from pathlib import Path

horizontal = 0
depth = 0
with open(Path(__file__).with_name("input.txt")) as input:
    for line in input:
        (direction, amount) = line.split()
        amount = int(amount)
        if direction == "forward":
            horizontal += amount
        elif direction == "down":
            depth += amount
        elif direction == "up":
            depth -= amount
        else:
            raise "Unknown direction!"

print(horizontal * depth)
