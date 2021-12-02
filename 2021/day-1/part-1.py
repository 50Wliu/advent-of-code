# Return number of times the current line > previous line

from pathlib import Path
import math

increases = 0
previousMeasurement = math.inf
with open(Path(__file__).with_name("input.txt")) as input:
    for line in input:
        measurement = int(line)
        if measurement > previousMeasurement:
            increases += 1

        previousMeasurement = measurement

print(increases)
