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
    traveled = 0
    last = perm[0]
    for i in range(1, len(perm)):
        traveled += distances[(last, perm[i])]
        last = perm[i]
    shortest = min(shortest, traveled)
    longest = max(longest, traveled)
print(shortest)
print(longest)
