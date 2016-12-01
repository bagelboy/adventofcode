#!/usr/bin/env python3

import re

if __name__ == '__main__':
    with open('input') as json_data:
        data = json_data.read()
    print(sum(map(lambda x: int(x.group(0)), re.finditer('-?[\d]+', data))))
