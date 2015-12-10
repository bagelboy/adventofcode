import re

start_value = '1113122113'

def expand(value):
    runs = map(lambda m: m.group(0), re.finditer(r'([\d])\1*', value))
    return ''.join(map(lambda r: '{}{}'.format(len(r), r[0]), runs))

value = start_value
for i in range(40):
    value = expand(value)

print(len(value))

for i in range(10):
    value = expand(value)

print(len(value))
