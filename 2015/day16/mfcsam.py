#!/usr/bin/env python3

class AuntSue:
    def __init__(self, number, posessions):
        self.number = number
        self.posessions = posessions

    def match_exact(self, criteria):
        score = 0
        for item, count in criteria.items():
            if item in self.posessions:
                if count != self.posessions[item]:
                    score = 0
                    break
                score += 1
        return score

    def match_range(self, criteria):
        score = 0
        for item, count in criteria.items():
            if item in self.posessions:
                has_count = self.posessions[item]
                score = self._adjust_score(score, item, count, self.posessions[item])
        return score

    def _adjust_score(self, score, item, reading, has):
        if (item == 'cats' or item == 'trees'):
            if reading < has:
                return score + 1
            return 0
        elif (item == 'pomeranians' or item == 'goldfish'):
            if reading > has:
                return score + 1
            return 0
        if reading == has:
            return score + 1
        return score

def parse_aunt(list):
    words = list.replace(':', '').replace(',', '').split()
    number = int(words[1])
    posessions = {}
    for i in range(2, len(words), 2):
        posessions[words[i]] = int(words[i+1])
    return AuntSue(number, posessions)

if __name__ == '__main__':
    with open('input') as data:
        aunts = [parse_aunt(line) for line in data]

    criteria = {
        'children': 3,
        'cats': 7,
        'samoyeds': 2,
        'pomeranians': 3,
        'akitas': 0,
        'vizslas': 0,
        'goldfish': 5,
        'trees': 3,
        'cars': 2,
        'perfumes': 1
    }

    print(sorted(aunts, key=lambda a: a.match_exact(criteria))[-1].number)
    print(sorted(aunts, key=lambda a: a.match_range(criteria))[-1].number)
