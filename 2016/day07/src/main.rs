use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug, PartialEq)]
enum Sequence<'a> {
    Supernet(&'a str),
    Hypernet(&'a str),
}

fn parse_sequences(ip: &str) -> Vec<Sequence> {
    ip.split(|c| c == '[' || c == ']')
        .enumerate()
        .map(|(i, slice)| match i % 2 == 0 {
            true => Sequence::Supernet(slice),
            _ => Sequence::Hypernet(slice),
        })
        .collect()
}

#[test]
fn parse_sequences_test() {
    assert_eq!(parse_sequences(&"abba[mnop]qrst"),
               vec![Sequence::Supernet("abba"),
                    Sequence::Hypernet("mnop"),
                    Sequence::Supernet("qrst")]);
}

fn has_abba(sequence: &str) -> bool {
    for bytes in sequence.as_bytes().windows(4) {
        if bytes[0] != bytes[1] && bytes[0] == bytes[3] && bytes[1] == bytes[2] {
            return true;
        }
    }
    false
}

#[test]
fn has_abba_test() {
    assert!(has_abba("abba"));
    assert!(!has_abba("asdf"));
    assert!(!has_abba("lol"));
    assert!(has_abba("asdfxzzxasdf"));
}

fn has_aba(sequence: &str) -> bool {
    for bytes in sequence.as_bytes().windows(3) {
        if bytes[0] != bytes[1] && bytes[0] == bytes[2] {
            return true;
        }
    }
    false
}

#[test]
fn has_aba_test() {
    assert!(has_aba("asdflolasdf"));
    assert!(!has_aba("qwertyuiop"));
    assert!(has_aba("lol"));
}

fn supports_tls(segments: &Vec<Sequence>) -> bool {
    let mut supported = false;
    for segment in segments {
        match *segment {
            Sequence::Hypernet(slice) => {
                if has_abba(slice) {
                    return false;
                }
            }
            Sequence::Supernet(slice) => {
                if has_abba(slice) {
                    supported = true;
                }
            }
        }
    }
    supported
}

#[test]
fn supports_tls_test() {
    assert!(supports_tls(&vec![Sequence::Supernet("abba"),
                               Sequence::Hypernet("mnop"),
                               Sequence::Supernet("qrst")]));
    assert!(!supports_tls(&vec![Sequence::Supernet("abcd"),
                                Sequence::Hypernet("bddb"),
                                Sequence::Supernet("xyyx")]));
    assert!(!supports_tls(&vec![Sequence::Supernet("aaaa"),
                                Sequence::Hypernet("qwer"),
                                Sequence::Supernet("tyui")]));
    assert!(supports_tls(&vec![Sequence::Supernet("ioxxoj"),
                               Sequence::Hypernet("asdfgh"),
                               Sequence::Supernet("zxcvbn")]));
}

fn main() {
    let count = BufReader::new(File::open("input").expect("cannot open input"))
        .lines()
        .fold(0,
              |count, l| match supports_tls(&parse_sequences(&l.unwrap())) {
                  true => count + 1,
                  _ => count,
              });
    println!("{}", count);
}
