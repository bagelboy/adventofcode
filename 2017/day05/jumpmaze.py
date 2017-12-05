#!/usr/bin/env python3


def escape_jump_maze(offsets):
    steps = 0
    position = 0
    while position in range(len(offsets)):
        steps += 1
        old_position = position
        position += offsets[position]
        offsets[old_position] += 1
    return steps


def escape_strange_jump_maze(offsets):
    steps = 0
    position = 0
    while position in range(len(offsets)):
        steps += 1
        old_position = position
        position += offsets[old_position]
        if offsets[old_position] < 3:
            offsets[old_position] += 1
        else:
            offsets[old_position] -= 1
    return steps


with open('input') as input:
    offsets = [int(x) for x in input.read().splitlines()]

print('maze one:', escape_jump_maze(offsets[:]), 'steps')
print('maze two:', escape_strange_jump_maze(offsets[:]), 'steps')
