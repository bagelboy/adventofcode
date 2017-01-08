use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn supports_tls(ip: &str) -> bool {
    let mut supported = false;
    let mut in_hypernet = false;
    for (i, c) in ip.chars().enumerate() {
        if c == '[' {
            if in_hypernet {
                panic!("'[' encountered in hypernet sequence]");
            }
            in_hypernet = true;
        } else if c == ']' {
            if !in_hypernet {
                panic!("']' encountered while not in hypernet sequence");
            }
            in_hypernet = false;
        } else if i > ip.len() - 4 {
            break;
        } else {
            let slice: &str = &ip[i..i + 4];
            if slice.contains('[') || slice.contains(']') {
                // not a clean slice
                continue;
            }

            let bytes = slice.as_bytes();
            let has_abba = bytes[0] != bytes[1] && bytes[0] == bytes[3] && bytes[1] == bytes[2];
            if has_abba && in_hypernet {
                return false;
            }
            supported |= has_abba;
        }
    }
    supported
}

#[test]
fn supports_tls_test() {
    assert!(supports_tls(&"abba[mnop]qrst"));
    assert!(!supports_tls(&"abcd[bddb]xyyx"));
    assert!(!supports_tls(&"aaaa[qwer]tyui"));
    assert!(supports_tls(&"ioxxoj[asdfgh]zxcvbn"));
}

fn main() {
    let count = BufReader::new(File::open("input").expect("cannot open input"))
        .lines()
        .fold(0, |count, l| match supports_tls(&l.unwrap()) {
            true => count + 1,
            _ => count,
        });
    println!("{}", count);
}
