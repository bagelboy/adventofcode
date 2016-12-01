#!/usr/bin/env python3

from collections import namedtuple

Dimension = namedtuple('Dimension', ['length', 'width', 'height'])

def paper_needed(dimension):
    side_areas = ((dimension.length * dimension.width),
                  (dimension.width * dimension.height),
                  (dimension.height * dimension.length))
    surface_area = 2 * sum(side_areas)
    return surface_area + min(side_areas)

def ribbon_needed(dimension):
    perimeters = ((2 * (dimension.length + dimension.width)),
                  (2 * (dimension.width + dimension.height)),
                  (2 * (dimension.height + dimension.length)))
    volume = dimension.length * dimension.width * dimension.height
    return min(perimeters) + volume

def run_tests():
    ex1 = Dimension(2, 3, 4)
    assert(paper_needed(ex1) == 58)
    assert(ribbon_needed(ex1) == 34)
    ex2 = Dimension(1, 1, 10)
    assert(paper_needed(ex2) == 43)
    assert(ribbon_needed(ex2) == 14)

total_paper = total_ribbon = 0
with open('input.txt') as input:
    for line in input:
        # convert string dimension (1x2x3) to tuple (1, 2, 3)
        dimension = Dimension._make(map(int, line.rstrip().split('x')))
        total_paper += paper_needed(dimension)
        total_ribbon += ribbon_needed(dimension)

run_tests()
print('Wrapping paper:', total_paper)
print('Ribbon:', total_ribbon)
