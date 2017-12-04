#!/usr/bin/env python3


def is_valid_passphrase(passphrase):
    all_words = [x for x in passphrase.split()]
    unique_words = set(all_words)
    return len(unique_words) == len(all_words)


def is_valid_anagram_passphrase(passphrase):
    all_words = [x for x in passphrase.split()]
    unique_words = set([''.join(sorted(x)) for x in all_words])
    return len(unique_words) == len(all_words)


if __name__ == '__main__':
    valid_passphrases = 0
    valid_anagram_passphrases = 0

    with open('input') as input:
        for line in input:
            passphrase = line.rstrip()
            if is_valid_passphrase(passphrase):
                valid_passphrases += 1
            if is_valid_anagram_passphrase(passphrase):
                valid_anagram_passphrases += 1

    print('valid passphrases:', valid_passphrases)
    print('valid anagram passphrases:', valid_anagram_passphrases)
