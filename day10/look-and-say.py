import re

start_value = '1113122113'

def expand(value):
    runs = []
    for m in re.finditer(r'([\d])\1*', value):
        runs.append(m.group(0))
    processed = []
    for last, next in zip(runs, runs):
        processed.append('{}{}'.format(len(last), last[0]))

    return ''.join(processed)

value = start_value
for i in range(40):
    value = expand(value)

print(len(value))

for i in range(10):
    value = expand(value)

print(len(value))
