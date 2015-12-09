def follow_directions(directions):
    floor = 0
    basement_position = 0
    for position, direction in enumerate(directions, start=1):
        if direction == '(':
            floor += 1
        elif direction == ')':
            floor -= 1
        if floor < 0 and basement_position == 0:
            basement_position = position
    return (floor, basement_position)

def run_tests():
    assert(follow_directions('(())') == (0, 0))
    assert(follow_directions('()()') == (0, 0))
    assert(follow_directions('))(((((') == (3, 1))
    assert(follow_directions(')())())') == (-3, 1))

run_tests()

with open('input.txt') as input:
    directions = input.read()

floor, basement_position = follow_directions(directions)
print('The instructions took Santa to floor', floor)
if basement_position > 0:
    print('Santa entered the basement at position', basement_position)
