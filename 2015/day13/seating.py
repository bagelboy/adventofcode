#!/usr/bin/env python3

from collections import defaultdict
import itertools

def parse_relationships(raw_data):
    relationships = defaultdict(dict)
    for line in raw_data:
        words = line.rstrip('\r\n.').split()
        person, gain_or_lose, how_much, next_to = words[0], words[2], \
            int(words[3]), words[-1]
        if gain_or_lose == 'lose':
            how_much = -how_much
        relationships[person][next_to] = how_much
    return relationships

def find_happiest_arrangement(relationships):
    happiest = -2 ** 32
    for arrangement in itertools.permutations(relationships.keys()):
        happiness = 0 # base
        for index, person in enumerate(arrangement):
            neighbors = relationships[person]
            left = neighbors[arrangement[(index-1) % len(arrangement)]]
            right = neighbors[arrangement[(index+1) % len(arrangement)]]
            happiness += (left + right)
        happiest = max(happiest, happiness)
    return happiest

def run_tests():
    with open('test_input') as raw_data:
        relationships = parse_relationships(raw_data)
    assert(find_happiest_arrangement(relationships) == 330)

if __name__ == '__main__':
    run_tests()
    with open('input') as raw_data:
        relationships = parse_relationships(raw_data)

    # part one
    print(find_happiest_arrangement(relationships))

    # part two
    my_name = 'Mike'
    my_relationships = relationships[my_name]
    for name in relationships:
        relationships[my_name][name] = 0
        relationships[name][my_name] = 0
    print(find_happiest_arrangement(relationships))
