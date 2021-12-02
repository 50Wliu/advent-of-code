# Return number of times a sliding window of 3 measurements
# is greater than the previous window

from pathlib import Path
import math

increases = 0
previousMeasurements = [math.inf, math.inf, math.inf] # From oldest to newest
with open(Path(__file__).with_name("input.txt")) as input:
    for line in input:
        measurement = int(line)

        oldWindowSum = sum(previousMeasurements)
        previousMeasurements.pop(0)
        previousMeasurements.append(measurement)
        newWindowSum = sum(previousMeasurements)

        if newWindowSum > oldWindowSum:
            increases += 1

print(increases)
