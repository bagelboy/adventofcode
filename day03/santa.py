class Santa:
    def __init__(self, start, visited):
        self.visited = visited
        self.x, self.y = start
        # drop a present at start
        self.move('')

    def send_directions(self, directions):
        for d in directions:
            self.move(d)
        return self

    def move(self, direction):
        if direction == '^':
            self.y += 1 # north
        elif direction == '>':
            self.x += 1 # east
        elif direction == 'v':
            self.y -=1 # south
        elif direction == '<':
            self.x -=1 # west
        self.visited.add((self.x, self.y))

def dispatch_santas(count, directions):
    visited = set()
    santas = [Santa((0, 0), visited) for _ in range(count)]
    for i, direction in enumerate(directions):
        santas[i % count].move(direction)
    return len(visited)

def run_tests():
    assert(dispatch_santas(1, '') == 1)
    assert(dispatch_santas(1, '>') == 2)
    assert(dispatch_santas(1, '^v^v^v^v^v') == 2)
    assert(dispatch_santas(2, '^v^v^v^v^v') == 11)
    assert(dispatch_santas(2, '^v') == 3)
    assert(dispatch_santas(2, '^>v<') == 3)

run_tests()
with open('input') as input:
    directions = input.read()

print('One Santa:  {} houses visited'.format(dispatch_santas(1, directions)))
print('Two Santas: {} houses visited'.format(dispatch_santas(2, directions)))
