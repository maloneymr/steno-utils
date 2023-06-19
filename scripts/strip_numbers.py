import sys
import json

with open(sys.argv[1]) as infile:
    dictionary = json.load(infile)

outlines_to_delete = []

for outline in dictionary:
    if any(digit in outline for digit in '0123456789'):
        outlines_to_delete.append(outline)

for outline in outlines_to_delete:
    del dictionary[outline]

json.dump(dictionary, sys.stdout, indent=4)
print()
