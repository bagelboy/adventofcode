#!/usr/bin/env python3

def has_repeated_pair(string):
    for i in range(len(string)):
        for j in range(len(string)-2, i+1, -1):
            if string[i] == string[j] and string[i+1] == string[j+1]:
                return True
    return False

def has_repeated_char(string):
    for i in range(0, len(string)-2):
        if string[i] == string[i+2]:
            return True
    return False

def is_nice(string):
    return has_repeated_pair(string) and has_repeated_char(string)

assert(is_nice('qjhvhtzxzqqjkmpb'))
assert(is_nice('xxyxx'))
assert(not is_nice('uurcxstgmygtbstg'))
assert(not is_nice('haegwjzuvuyypxyu'))
assert(not is_nice('ieodomkazucvgmuy'))

nice_strings = 0
with open('input') as input:
    for line in input:
        string = line.rstrip()
        if is_nice(string):
            nice_strings += 1

print('Nice strings: ', nice_strings)
