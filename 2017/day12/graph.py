def read_input():
    connections = []
    with open('input') as input:
        for line in input:
            values = line.rstrip().split(' <-> ', 2)
            other_nodes = values[1].split(', ')
            connections.append([int(x) for x in other_nodes])
    return connections


neighbors = read_input()


def discover_group(start, neighbors, visited):
    nodes = [start]
    group = []
    while len(nodes) > 0:
        current = nodes.pop()
        if current in visited:
            continue
        for n in neighbors[current]:
            nodes.append(n)
        visited.add(current)
        group.append(current)
    return group


visited = set()
groups = [discover_group(0, neighbors, visited)]
print('Programs in group 0:', len(groups[0]))

for i in range(len(neighbors)):
    if i not in visited:
        groups.append(discover_group(i, neighbors, visited))

print('total groups:', len(groups))
