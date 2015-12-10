#!/usr/bin/env python3

from enum import Enum
from collections import namedtuple

LocationTuple = namedtuple('LocationTuple', field_names=['x', 'y'])

rows = columns = 1000

lights = [0 for x in range(rows * columns)]

def read_action(line):
    if line.startswith('toggle'):
        return 2
    elif line.startswith('turn on'):
        return 1
    return -1

def read_directions(line):
    parts = line.split()
    if parts[0] == 'turn':
        start = 2
        end = 4
    else:
        start = 1
        end = 3
    start = LocationTuple._make(map(int, parts[start].split(',')))
    end = LocationTuple._make(map(int, parts[end].split(',')))
    return (start, end)

# part two
def adjust_brightness(action, start_point, end_point):
    for y in range(start_point.y, end_point.y + 1):
        for x in range(start_point.x, end_point.x + 1):
            index = x + y * columns
            lights[index] = max(0, lights[index] + action)

with open('input') as input:
    for line in input:
        action = read_action(line)
        start, end = read_directions(line)
        assert(start.x <= end.x and start.y <= end.y)
        adjust_brightness(action, start, end)

print(sum(lights), 'lights are lit')
