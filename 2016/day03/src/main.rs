use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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

    println!("{} possible triangles", valid);
}

#[test]
fn test_possible_triangles() {
    assert_eq!(false, Triangle(5, 10, 25).is_possible());
    assert_eq!(true, Triangle(15, 25, 30).is_possible());
}
