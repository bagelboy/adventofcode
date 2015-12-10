#!/usr/bin/env python3

import itertools

locations = set()
distances = {}

with open('input') as input:
    for line in input:
        pieces = line.rstrip().split()
        origin, destination, distance = pieces[0], pieces[2], int(pieces[4])
        locations.add(origin)
        locations.add(destination)
        distances[(origin, destination)] = distance
        distances[(destination, origin)] = distance

shortest = sum(distances.values()) * 1000
longest = 0
for perm in itertools.permutations(locations):
    traveled = sum(map(lambda pair: distances[pair], zip(perm, perm[1:])))
    shortest = min(shortest, traveled)
    longest = max(longest, traveled)
print(shortest)
print(longest)
