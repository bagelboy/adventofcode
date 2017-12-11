#!/usr/bin/env python3


def follow_directions(directions):
    x, y = 0, 0
    furthest = 0
    for direction in directions:
        if direction == 'ne':
            x += 1
            y -= 1
        elif direction == 'sw':
            x -= 1
            y += 1
        elif direction == 'nw':
            x -= 1
        elif direction == 'se':
            x += 1
        elif direction == 'n':
            y -= 1
        elif direction == 's':
            y += 1
        else:
            raise ValueError('invalid direction: ' + direction)
        distances = [abs(x), abs(y), abs(-(x + y))]
        distance = max(distances)
        furthest = max(furthest, distance)
    return (distance, furthest)


assert follow_directions(['ne', 'ne', 'ne'])[0] == 3
assert follow_directions(['ne', 'ne', 'sw', 'sw'])[0] == 0
assert follow_directions(['ne', 'ne', 's', 's'])[0] == 2
assert follow_directions(['se', 'sw', 'se', 'sw', 'sw'])[0] == 3


def main():
    with open('input') as input:
        directions = input.read().rstrip().split(',')

    distance, furthest = follow_directions(directions)
    print('fewest steps:', distance)
    print('furthest from start:', furthest)


if __name__ == '__main__':
    main()
