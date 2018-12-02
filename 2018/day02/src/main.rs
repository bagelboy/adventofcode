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

fn main() {
    let input = fs::read_to_string("input").expect("cannot read input");
    let ids: Vec<_> = input.lines().collect();
    println!("checksum: {}", compute_checksum(&ids));
}
