extern crate itertools;

use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn compute_checksum(ids: &[&str]) -> u32 {
    let mut doubles = 0;
    let mut triples = 0;
    let mut seen = HashMap::new();
    for id in ids {
        seen.clear();
        for letter in id.chars() {
            seen.entry(letter)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        if seen.values().any(|count| *count == 2) {
            doubles += 1;
        }
        if seen.values().any(|count| *count == 3) {
            triples += 1;
        }
    }
    doubles * triples
}

#[test]
fn test_compute_checksum() {
    assert_eq!(
        12,
        compute_checksum(&["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"])
    );
}

fn compare(left: &str, right: &str) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let b1 = left.as_bytes();
    let b2 = right.as_bytes();
    let mut differences = 0;
    for i in 0..b1.len() {
        if b1[i] != b2[i] {
            differences += 1;
        }
        if differences > 1 {
            return false;
        }
    }
    true
}

fn find_common_letters<'a>(ids: &'a [&str]) -> Option<String> {
    for (left, right) in ids.iter().tuple_combinations() {
        if compare(left, right) {
            return Some(
                left.chars()
                    .zip(right.chars())
                    .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                    .collect::<String>(),
            );
        }
    }
    None
}

#[test]
fn test_find_common_letters() {
    assert_eq!(
        Some("fgij".to_string()),
        find_common_letters(&["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"])
    );
}

fn main() {
    let input = fs::read_to_string("input").expect("cannot read input");
    let ids: Vec<_> = input.lines().collect();
    println!("checksum: {}", compute_checksum(&ids));
    println!("common letters: {}", find_common_letters(&ids).unwrap());
}
