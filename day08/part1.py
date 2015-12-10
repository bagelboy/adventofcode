#!/usr/bin/env python3

import re

tokens = r'(?P<CHAR>\\x[0-9a-f][0-9a-f])|(?P<QUOTE>\\\")|(?P<BS>\\\\)'

def memory_characters(string):
    length = len(string)
    if string.startswith(r'"') and string.endswith(r'"'):
        length -= 2
    for match in re.finditer(tokens, string):
        token = match.lastgroup
        if token == 'QUOTE' or token == 'BS':
            length -= 1 # from 2 chars to 1
        elif token == 'CHAR':
            length -= 3 # from 4 chars to 1
    return length

total = 0
with open('input') as input:
    for line in input:
        string = line.rstrip()
        total += len(string)
        total -= memory_characters(string)

print(total)
