#!/usr/bin/env python3

import re

sub = { 'a': 'b', 'b': 'c', 'c': 'd', 'd': 'e', 'e': 'f', 'f': 'g',
        'g': 'h', 'h': 'i', 'i': 'j', 'j': 'k', 'k': 'l', 'l': 'm',
        'm': 'n', 'n': 'o', 'o': 'p', 'p': 'q', 'q': 'r', 'r': 's',
        's': 't', 't': 'u', 'u': 'v', 'v': 'w', 'w': 'x', 'x': 'y',
        'y': 'z' }

def has_straight(password):
    for i in range(len(password)-2):
        if (password[i] in sub and password[i+1] in sub
                and sub[password[i]] == password[i+1]
                and sub[password[i+1]] == password[i+2]):
            return True
    return False

def has_no_bad_chars(password):
    return bool(re.search(r'[iol]', password)) == False

def has_pairs(password):
    return bool(re.search(r'([a-z])\1.*([a-z])\2', password)) == True

def is_valid_password(password):
    return (has_straight(password) and
            has_no_bad_chars(password) and
            has_pairs(password))

def increment_password(old_password):
    letters = list(old_password)
    for i in range(len(letters)-1, -1, -1):
        if letters[i] != 'z':
            letters[i] = sub[letters[i]]
            break
        letters[i] = 'a'
    return ''.join(letters)

def next_password(old_password):
    next = old_password
    while True:
        next = increment_password(next)
        if is_valid_password(next):
            return next
    return None

def run_tests():
    assert(is_valid_password('ghjaabcc'))
    assert(is_valid_password('abcdffaa'))
    assert(not is_valid_password('hijklmmn'))
    assert(not is_valid_password('abbceffg'))
    assert(not is_valid_password('abbcegjk'))
    assert(next_password('abcdefgh') == 'abcdffaa')
    assert(next_password('ghijklmn') == 'ghjaabcc')

# run_tests()
first = next_password('vzbxkghb')
print(first)
print(next_password(first))
