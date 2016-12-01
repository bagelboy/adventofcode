#!/usr/bin/env python3

from hashlib import md5

def find_number(key, pattern):
    found = False
    key_bytes = key.encode()
    number = 1
    while True:
        hasher = md5(key_bytes)
        hasher.update(str(number).encode())
        if hasher.hexdigest().startswith(pattern):
            return number
        number += 1

# TODO: these tests take too long to run every time
# assert(find_number('abcdef', '0' * 5) == 609043)
# assert(find_number('pqrstuv', '0' * 5) == 1048970)

key = 'iwrupvqb'

print('Five zeroes:', find_number(key, '0' * 5))
print('Six zeroes: ', find_number(key, '0' * 6))
