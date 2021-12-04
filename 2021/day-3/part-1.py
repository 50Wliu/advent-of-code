# Determines gamma & epsilon values from diagnostics
# and returns power consumption as gamma * epsilon

from pathlib import Path

with open(Path(__file__).with_name("input.txt")) as input:
    # Create arrays of length (whatever the length of the first line is)
    # readline includes newline, so subtract 1
    expectedDiagnosticsPerLine = len(input.readline()[:-1])
    numOnes = [0] * expectedDiagnosticsPerLine
    gammaValues = [0] * expectedDiagnosticsPerLine
    epsilonValues = [0] * expectedDiagnosticsPerLine

    # Reset file pointer
    input.seek(0)

    lines = 0
    for line in input:
        lines += 1
        if len(line[:-1]) != expectedDiagnosticsPerLine:
            raise f"Line {lines} has an unexpected number of diagnostics"

        for pos, char in enumerate(line[:-1]):
            if char == "1":
                numOnes[pos] += 1

    majorityNeeded = lines // 2 if lines % 2 == 0 else lines // 2 + 1
    for pos, count in enumerate(numOnes):
        if count >= majorityNeeded:
            gammaValues[pos] = 1
            epsilonValues[pos] = 0
        else:
            gammaValues[pos] = 0
            epsilonValues[pos] = 1

    # Convert to decimal
    gamma = int(''.join([str(value) for value in gammaValues]), 2)
    epsilon = int(''.join([str(value) for value in epsilonValues]), 2)

    print(gamma * epsilon)
