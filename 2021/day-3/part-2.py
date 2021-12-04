# Determines oxygen generator & CO2 scrubbing values from diagnostics
# and returns life support rating as oxygen generator * CO2 scrubbing

from pathlib import Path

lines = []
with open(Path(__file__).with_name("input.txt")) as input:
    lines = input.readlines()

validOxygenLineNumbers = set(range(len(lines)))
validCO2LineNumbers = validOxygenLineNumbers.copy()

# Newline is included, so subtract 1
for pos in range(len(lines[0]) - 1):
    if len(validOxygenLineNumbers) > 1:
        oneLines = [lineNumber for lineNumber in validOxygenLineNumbers if lines[lineNumber][pos] == "1"]
        oxygenMajorityNeeded = len(validOxygenLineNumbers) // 2 if len(validOxygenLineNumbers) % 2 == 0 else len(validOxygenLineNumbers) // 2 + 1
        validOxygenLineNumbers.intersection_update(oneLines if len(oneLines) >= oxygenMajorityNeeded else validOxygenLineNumbers.difference(oneLines))

    if len(validCO2LineNumbers) > 1:
        oneLines = [lineNumber for lineNumber in validCO2LineNumbers if lines[lineNumber][pos] == "1"]
        co2MinorityNeeded = len(validCO2LineNumbers) // 2 if len(validCO2LineNumbers) % 2 == 0 else len(validCO2LineNumbers) // 2 + 1
        validCO2LineNumbers.intersection_update(oneLines if len(oneLines) < co2MinorityNeeded else validCO2LineNumbers.difference(oneLines))

    if len(validOxygenLineNumbers) == 1 and len(validCO2LineNumbers) == 1:
        break

# Convert to decimal
oxygenGenerator = int(''.join([str(value) for value in lines[validOxygenLineNumbers.pop()][:-1]]), 2)
co2Scrubbing = int(''.join([str(value) for value in lines[validCO2LineNumbers.pop()][:-1]]), 2)

print(oxygenGenerator * co2Scrubbing)
