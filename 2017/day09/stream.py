#!/usr/bin/env python3


def process_stream(input):
    is_garbage = False
    in_group = False
    total_score = 0
    depth = 0
    garbage_count = 0
    it = iter(input)
    for c in it:
        if is_garbage:
            if c == '!':
                # ignore next char
                next(it)
            elif c == '>':
                is_garbage = False
            else:
                garbage_count += 1
        elif c == '{':
            depth += 1
        elif c == '}':
            total_score += depth
            depth -= 1
        elif c == '<':
            is_garbage = True

    return (total_score, garbage_count)


if __name__ == '__main__':
    # part one
    assert (process_stream('{}')[0] == 1)
    assert (process_stream('{{{}}}')[0] == 6)
    assert (process_stream('{{},{}}')[0] == 5)
    assert (process_stream('{{{},{},{{}}}}')[0] == 16)
    assert (process_stream('{<a>,<a>,<a>,<a>}')[0] == 1)
    assert (process_stream('{{<ab>},{<ab>},{<ab>},{<ab>}}')[0] == 9)
    assert (process_stream('{{<!!>},{<!!>},{<!!>},{<!!>}}')[0] == 9)
    assert (process_stream('{{<a!>},{<a!>},{<a!>},{<ab>}}')[0] == 3)

    # part two
    assert (process_stream('<>')[1] == 0)
    assert (process_stream('<random characters>')[1] == 17)
    assert (process_stream('<<<<>')[1] == 3)
    assert (process_stream('<{!>}>')[1] == 2)
    assert (process_stream('<!!>')[1] == 0)
    assert (process_stream('<!!!>>')[1] == 0)
    assert (process_stream('<{o"i!a,<{i<a>')[1] == 10)

    with open('input') as input:
        total_score, garbage_chars = process_stream(input.read().rstrip())
        print('total score:', total_score)
        print('garbage chars:', garbage_chars)
