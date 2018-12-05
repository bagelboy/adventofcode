use std::fs;

fn reduce(polymer: &[u8]) -> String {
    let mut input = polymer.clone().to_vec();
    while input.len() > 1 {
        let old_size = input.len();
        for i in 0..input.len() - 1 {
            let current = input[i];
            let next = input[i + 1];
            if (current.is_ascii_lowercase() && next == current.to_ascii_uppercase())
                || (current.is_ascii_uppercase() && next == current.to_ascii_lowercase())
            {
                input.remove(i);
                input.remove(i);
                break;
            }
        }
        if input.len() == old_size {
            break;
        }
    }

    String::from_utf8(input).expect("this should still be valid utf-8")
}

fn full_reduce(polymer: &[u8]) -> String {
    let mut input = polymer.clone().to_vec();

    while let Some(location) = find_unit(&input) {
        let first = input[location];
        let second = input[location + 1];
        let mut i = input.len() - 1;
        while i != 0 {
            if input[i] == second && input[i - 1] == first {
                println!("removing {}", input[i] as char);
                input.remove(i);
                println!("removing {}", input[i] as char);
                input.remove(i);
                i -= 2;
                continue;
            }
            i -= 1;
        }
    }

    String::from_utf8(input).expect("this should work")
}

fn find_unit(polymer: &[u8]) -> Option<usize> {
    let mut i = 0;
    while i < polymer.len() - 1 {
        let current = polymer[i];
        let next = polymer[i + 1];
        if current != next
            && ((current.is_ascii_lowercase() && current.to_ascii_uppercase() == next)
                || (current.is_ascii_uppercase() && current.to_ascii_lowercase() == next))
        {
            return Some(i);
        }
        i += 1;
    }
    None
}

#[test]
fn test_find_unit() {
    assert_eq!(Some(3), find_unit("xxxaA".as_bytes()));
}

#[test]
fn test_reduce() {
    assert_eq!("", reduce("aA".as_bytes()));
    assert_eq!("", reduce("abBA".as_bytes()));
    assert_eq!("abAB", reduce("abAB".as_bytes()));
    assert_eq!("aabAAB", reduce("aabAAB".as_bytes()));
    assert_eq!("dabCBAcaDA", reduce("dabAcCaCBAcCcaDA".as_bytes()));
}

#[test]
fn test_full_reduce() {
    assert_eq!("abCBAc", full_reduce("dabAcCaCBAcCcaDA".as_bytes()));
}

fn main() {
    let mut input = fs::read("input").expect("cannot read input");
    println!("{:?}", input[input.len() - 1]);
    // trim newline
    input.pop();
    println!("{:?}", input[input.len() - 1]);
    //println!("{}, {}", input[input.len() - 2] as char, input[input.len() - 1] as char);

    println!("{}", reduce(&input).len());
    println!("{}", full_reduce(&input).len());
}
