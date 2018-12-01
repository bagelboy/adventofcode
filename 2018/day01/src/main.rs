use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Vec<i32> {
    let mut buffer = String::new();
    let mut reader = BufReader::new(File::open("input").expect("cannot read input"));
    let mut numbers = Vec::new();

    while reader.read_line(&mut buffer).unwrap() > 0 && buffer.len() > 0 {
        numbers.push(buffer.trim_end().parse().expect("should be a number"));
        buffer.clear();
    }
    numbers
}

fn find_first_repeating_frequency(adjustments: &[i32]) -> i32 {
    let mut frequency = 0;
    let mut seen = HashSet::new();
    seen.insert(frequency);
    for adjustment in adjustments.iter().cycle() {
        frequency += adjustment;
        if !seen.insert(frequency) {
            return frequency;
        }
    }
    unreachable!()
}

#[test]
fn test_find_first_repeating_frequency() {
    assert_eq!(0, find_first_repeating_frequency(&[1, -1]));
    assert_eq!(10, find_first_repeating_frequency(&[3, 3, 4, -2, -4]));
    assert_eq!(5, find_first_repeating_frequency(&[-6, 3, 8, 5, -6]));
    assert_eq!(14, find_first_repeating_frequency(&[7, 7, -2, -7, -4]));
}

fn main() {
    let input = read_input();
    println!("initial frequency: {}", input.iter().sum::<i32>());
    println!(
        "calibrated frequency: {}",
        find_first_repeating_frequency(&input)
    );
}
