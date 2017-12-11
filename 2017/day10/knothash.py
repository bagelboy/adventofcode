#!/usr/bin/env python3
import sys


def knot_hash(elements, lengths, position=0, skip_size=0):
    count = len(elements)
    for length in lengths:
        for i in range(length // 2):
            index = (position + i) % count
            swap_index = ((position + (length - 1)) - i) % count
            elements[index], elements[swap_index] = elements[
                swap_index], elements[index]
        position = (position + length + skip_size) % count
        skip_size += 1
    return (position, skip_size)


def do_test():
    elements = list(range(5))
    lengths = [3, 4, 1, 5]
    knot_hash(elements, lengths, 0, 0)
    assert elements[0] * elements[1] == 12


def dense_hash(elements):
    assert len(elements) % 16 == 0
    hash = 0
    for e in elements:
        hash ^= e
    return hash


def full_hash(elements, lengths):
    lengths += [17, 31, 73, 47, 23]
    position, skip_size = 0, 0
    for round in range(64):
        position, skip_size = knot_hash(elements, lengths, position, skip_size)
    assert len(elements) % 16 == 0
    sparse_hashes = [
        dense_hash(elements[x * 16:((x + 1) * 16)]) for x in range(16)
    ]
    return ''.join(['{:02x}'.format(x) for x in sparse_hashes])


if __name__ == '__main__':
    do_test()

    with open('input') as input:
        raw_input = input.read().rstrip()

    # part one
    elements = list(range(256))
    lengths = [int(x) for x in raw_input.split(',')]
    position, skip_size = knot_hash(elements, lengths)
    print('product of first two numbers:', elements[0] * elements[1])

    # more weird tests
    assert '{:02x}'.format(64) == '40'
    assert '{:02x}'.format(7) == '07'
    assert '{:02x}'.format(255) == 'ff'

    # part two
    elements = list(range(256))
    lengths = [ord(x) for x in raw_input]
    print('knot hash:', full_hash(elements, lengths))
