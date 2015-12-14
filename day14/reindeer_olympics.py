#!/usr/bin/env python3

class Reindeer:
    def __init__(self, name, velocity, fly_time, rest_time):
        self.name = name
        self.velocity = velocity
        self.fly_time = fly_time
        self.rest_time = rest_time

        self.is_flying = True
        self.distance_flown = 0
        self.time_flown = 0
        self.time_rested = 0
        self.points_earned = 0

    def increment_time(self):
        if self.is_flying:
            self.distance_flown += self.velocity
            self.time_flown += 1
            if self.time_flown == self.fly_time:
                self.is_flying = False
                self.time_flown = 0
        else:
            self.time_rested += 1
            if self.time_rested == self.rest_time:
                self.is_flying = True
                self.time_rested = 0

def parse_reindeer(raw_data):
    def parse_line(line):
        data = line.split()
        name, velocity, fly_time, rest_time = data[0], int(data[3]), \
            int(data[6]), int(data[-2])
        return Reindeer(name, velocity, fly_time, rest_time)
    return [parse_line(l) for l in raw_data]

def race(reindeer, seconds):
    for t in range(seconds):
        for r in reindeer:
            r.increment_time()
        lead_position = max([r.distance_flown for r in reindeer])
        for r in reindeer:
            if r.distance_flown == lead_position:
                r.points_earned += 1
    # 2-tuple of distance flown and number of points earned
    return (max([r.distance_flown for r in reindeer]),
            max([r.points_earned for r in reindeer]))

def run_tests():
    reindeer = (Reindeer('Comet', 14, 10, 127),
                Reindeer('Dancer', 16, 11, 162))
    distance, points = race(reindeer, 1000)
    assert(distance == 1120)
    assert(points == 689)

def main():
    run_tests()
    with open('input') as raw_data:
        reindeer = parse_reindeer(raw_data)
    distance, points = race(reindeer, 2503)
    print(distance)
    print(points)

if __name__ == '__main__':
    main()
