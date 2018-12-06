use std::fs;

fn react_polymer(buffer: &mut Vec<u8>) -> usize {
    while buffer.len() > 1 {
        let old_size = buffer.len();
        for i in 0..buffer.len() - 1 {
            let current = buffer[i];
            let next = buffer[i + 1];
            if (current.is_ascii_lowercase() && current.to_ascii_uppercase() == next)
                || (current.is_ascii_uppercase() && current.to_ascii_lowercase() == next)
            {
                buffer.drain(i..i + 2);
                break;
            }
        }
        if buffer.len() == old_size {
            break;
        }
    }
    buffer.len()
}

#[test]
fn test_react_polymer() {
    assert_eq!(0, react_polymer(&mut "aA".as_bytes().to_vec()));
    assert_eq!(0, react_polymer(&mut "abBA".as_bytes().to_vec()));
    assert_eq!(4, react_polymer(&mut "abAB".as_bytes().to_vec()));
    assert_eq!(6, react_polymer(&mut "aabAAB".as_bytes().to_vec()));
    assert_eq!(
        10,
        react_polymer(&mut "dabAcCaCBAcCcaDA".as_bytes().to_vec())
    );
}

fn find_best_reduce(polymer: &[u8]) -> usize {
    let mut shortest = polymer.len();
    for unit in b'A'..=b'Z' {
        let mut buffer = polymer.to_vec();
        let mut i = 0;
        while i < buffer.len() - 1 {
            if buffer[i].to_ascii_uppercase() == unit {
                buffer.drain(i..=i);
                continue;
            }
            i += 1;
        }
        let length = react_polymer(&mut buffer);

        if length < shortest {
            shortest = length;
        }
    }
    shortest
}

#[test]
fn test_find_best_reduce() {
    assert_eq!(4, find_best_reduce("dabAcCaCBAcCcaDA".as_bytes()));
}

fn main() {
    let mut input = fs::read("input").expect("cannot read input");
    input.pop(); // remove newline
    println!("part one: {}", react_polymer(&mut input.clone()));
    println!("part two: {}", find_best_reduce(&input));
}
