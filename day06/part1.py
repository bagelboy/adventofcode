from enum import Enum
from collections import namedtuple

LocationTuple = namedtuple('LocationTuple', field_names=['x', 'y'])

rows = columns = 1000

lights = [0 for x in range(rows * columns)]

def read_line(line):
    parts = line.split()
    if parts[0] == 'turn':
        action = parts[1]
        start = 2
        end = 4
    else:
        action = parts[0]
        start = 1
        end = 3
    start = LocationTuple._make(map(int, parts[start].split(',')))
    end = LocationTuple._make(map(int, parts[end].split(',')))
    return (action, start, end)

def set_lights(action, start_point, end_point):
    for y in range(start_point.y, end_point.y + 1):
        for x in range(start_point.x, end_point.x + 1):
            index = x * columns + y
            if action == 'off':
                lights[index] = False
            elif action == 'on':
                lights[index] = True
            else:
                lights[index] = not lights[index]

def adjust_brightness(action, start_point, end_point):
    for y in range(start_point.y, end_point.y + 1):
        for x in range(start_point.x, end_point.x + 1):
            index = x * columns + y
            if action == 'off':
                lights[index] -= 1
            elif action == 'on':
                lights[index] += 1
            else:
                lights[index] += 2

with open('input') as input:
    for line in input:
        action, start, end = read_line(line)
        assert(start.x <= end.x and start.y <= end.y)
        set_lights(action, start, end)

print(sum(lights), 'lights are lit')
