use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, PartialEq)]
struct Triangle(u32, u32, u32);

impl Triangle {
    fn is_possible(&self) -> bool {
        (self.0 + self.1) > self.2 && (self.1 + self.2) > self.0 && (self.0 + self.2) > self.1
    }
}

fn read_sides_from_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .take(3)
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn main() {
    {
        let valid = BufReader::new(File::open("input").expect("cannot open input"))
            .lines()
            .map(|l| {
                let line = l.expect("cannot read line");
                let sides = read_sides_from_line(&line);
                Triangle(sides[0], sides[1], sides[2])
            })
            .fold(0, |count, triangle| match triangle.is_possible() {
                true => count + 1,
                _ => count,
            });

        println!("part one: {} possible triangles", valid);
    }

    let mut triangles: Vec<Triangle> = Vec::new();
    {
        let mut lines = BufReader::new(File::open("input").expect("cannot open input")).lines();

        // FIXME: find a way to group lines by threes, probably with itertools or
        // something like iter.take(3).collect::Vec<String>().chunks(3)
        loop {
            let first = lines.next();
            if first.is_none() {
                break;
            }
            let second = lines.next().expect("missing line 2/3");
            let third = lines.next().expect("missing line 3/3");

            let r1 = read_sides_from_line(&first.unwrap().expect("cannot read line"));
            let r2 = read_sides_from_line(&second.expect("cannot read line"));
            let r3 = read_sides_from_line(&third.expect("cannot read line"));
            triangles.push(Triangle(r1[0], r2[0], r3[0]));
            triangles.push(Triangle(r1[1], r2[1], r3[1]));
            triangles.push(Triangle(r1[2], r2[2], r3[2]));
        }
    }
    let valid = triangles.iter().fold(0, |acc, t| match t.is_possible() {
        true => acc + 1,
        _ => acc,
    });
    println!("part two: {} possible triangles", valid);
}

#[test]
fn test_read_sides_from_line() {
    assert_eq!(vec![5, 10, 25], read_sides_from_line(&"    5   10   25"));
}

#[test]
fn test_possible_triangles() {
    assert_eq!(false, Triangle(5, 10, 25).is_possible());
    assert_eq!(true, Triangle(15, 25, 30).is_possible());
}
