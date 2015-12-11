#!/usr/bin/env python3

import re

sub = {'z': 'a', 's': 't', 'r': 's', 'k': 'l', 'p': 'q', 'o': 'p', 'l': 'm',
       'f': 'g', 'g': 'h', 'y': 'z', 'd': 'e', 'e': 'f', 'n': 'o', 'i': 'j',
       'h': 'i', 'j': 'k', 'q': 'r', 'm': 'n', 'w': 'x', 'b': 'c', 'a': 'b',
       'x': 'y', 't': 'u', 'v': 'w', 'u': 'v', 'c': 'd'}

def has_straight(password):
    for i in range(len(password)-2):
        if (ord(password[i]) == ord(password[i+1])-1 and
            ord(password[i+1]) == ord(password[i+2])-1):
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
