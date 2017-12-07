import itertools

class Tower():
    def __init__(self, name, weight, children):
        self.name = name
        self.weight = weight
        self.children = children

    def __str__(self):
        if len(self.children) > 0:
            return '{} ({}) -> {}'.format(self.name, self.weight, ', '.join(self.children))
        return '{} ({})'.format(self.name, self.weight)

class Node():
    def __init__(self, tower):
        self.parents = []

    def add_parent(self, parent):
        self.parents.append(parent)

input = '''pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
'''

def parse_towers(lines):
    towers = []
    for line in lines:
        parts = line.rstrip().split(' ', 3)
        name = parts[0]
        weight = int(parts[1][1:len(parts[1])-1])

        if len(parts) > 3:
            children = parts[3].split(', ')
        else:
            children = []
        towers.append(Tower(name, weight, children))
    return towers

def main():
    with open('input') as input:
        towers = parse_towers(input.read().splitlines())
    #raw = input.splitlines()
    #towers = parse_towers(input.splitlines())
    for t1 in towers:
        found = False
        # search for other tower with name in rhs
        for t2 in towers:
            if t1.name in t2.children:
                found = True
        if not found:
            print(t1)

if __name__ == '__main__':
    main()
