def has_three_vowels(string):
    vowels_seen = 0
    vowels = set(['a', 'e', 'i', 'o', 'u'])
    for char in string:
        if char in vowels:
            vowels_seen += 1
    return vowels_seen >= 3

def has_repeated_char(string):
    for position, char in enumerate(string):
        if position < len(string) - 1 and char == string[position + 1]:
            return True
    return False

def has_no_naughty_strings(string):
    naughty_strings = set(['ab', 'cd', 'pq', 'xy'])
    for naughty in naughty_strings:
        if string.find(naughty) != -1:
            return False
    return True

def is_nice(string):
    return (has_no_naughty_strings(string)
            and has_three_vowels(string)
            and has_repeated_char(string))

assert(is_nice('ugknbfddgicrmopn'))
assert(is_nice('aaa'))
assert(not is_nice('jchzalrnumimnmhp'))
assert(not is_nice('haegwjzuvuyypxyu'))
assert(not is_nice('dvszwmarrgswjxmb'))

nice_strings = 0
with open('input') as input:
    for line in input:
        string = line.rstrip()
        if is_nice(string):
            nice_strings += 1

print('Nice strings: ', nice_strings)
