use std::fs::File;
use std::io::Read;

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
fn test_parse_sequences() {
    assert_eq!(
        parse_sequences(&"abba[mnop]qrst"),
        vec![
            Sequence::Supernet("abba"),
            Sequence::Hypernet("mnop"),
            Sequence::Supernet("qrst"),
        ]
    );
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
fn test_has_abba() {
    assert!(has_abba("abba"));
    assert!(!has_abba("asdf"));
    assert!(!has_abba("lol"));
    assert!(has_abba("asdfxzzxasdf"));
}

fn get_aba(sequence: &str, abas: &mut Vec<(u8, u8)>) {
    for bytes in sequence.as_bytes().windows(3) {
        if bytes[0] != bytes[1] && bytes[0] == bytes[2] {
            abas.push((bytes[0], bytes[1]));
        }
    }
}

#[test]
fn test_get_aba() {
    let mut abas: Vec<(u8, u8)> = Vec::new();
    get_aba("asdflolasdfxyxasdf", &mut abas);
    assert_eq!(abas, vec![(b'l', b'o'), (b'x', b'y')]);
    abas.clear();
    get_aba("qwertyuiop", &mut abas);
    assert_eq!(abas, vec![]);
}

fn has_bab(aba: &(u8, u8), sequence: &str) -> bool {
    for bytes in sequence.as_bytes().windows(3) {
        if bytes[0] == aba.1 && bytes[2] == aba.1 && bytes[1] == aba.0 {
            return true;
        }
    }
    false
}

#[test]
fn test_has_bab() {
    assert!(has_bab(&(b'a', b'b'), "bab"));
    assert!(!has_bab(&(b'x', b'y'), "xyx"));
    assert!(has_bab(&(b'k', b'e'), "eke"));
    assert!(has_bab(&(b'z', b'b'), "bzb"));
}

fn supports_tls(segments: &Vec<Sequence>) -> bool {
    let mut supported = false;
    for segment in segments {
        match *segment {
            Sequence::Hypernet(slice) => if has_abba(slice) {
                return false;
            },
            Sequence::Supernet(slice) => if has_abba(slice) {
                supported = true;
            },
        }
    }
    supported
}

#[test]
fn test_supports_tls() {
    assert!(supports_tls(&vec![
        Sequence::Supernet("abba"),
        Sequence::Hypernet("mnop"),
        Sequence::Supernet("qrst"),
    ]));
    assert!(!supports_tls(&vec![
        Sequence::Supernet("abcd"),
        Sequence::Hypernet("bddb"),
        Sequence::Supernet("xyyx"),
    ]));
    assert!(!supports_tls(&vec![
        Sequence::Supernet("aaaa"),
        Sequence::Hypernet("qwer"),
        Sequence::Supernet("tyui"),
    ]));
    assert!(supports_tls(&vec![
        Sequence::Supernet("ioxxoj"),
        Sequence::Hypernet("asdfgh"),
        Sequence::Supernet("zxcvbn"),
    ]));
}

fn supports_ssl(segments: &Vec<Sequence>) -> bool {
    let mut aba_sequences: Vec<(u8, u8)> = Vec::new();
    for s in segments {
        match s {
            &Sequence::Supernet(s) => {
                get_aba(s, &mut aba_sequences);
            }
            _ => {}
        };
    }
    for s in segments {
        match s {
            &Sequence::Hypernet(s) => for aba in &aba_sequences {
                if has_bab(aba, s) {
                    return true;
                }
            },
            _ => {}
        };
    }
    false
}

#[test]
fn test_supports_ssl() {
    assert!(supports_ssl(&vec![
        Sequence::Supernet("aba"),
        Sequence::Hypernet("bab"),
        Sequence::Supernet("xyz"),
    ]));
    assert!(!supports_ssl(&vec![
        Sequence::Supernet("xyx"),
        Sequence::Hypernet("xyx"),
        Sequence::Supernet("xyx"),
    ]));
    assert!(supports_ssl(&vec![
        Sequence::Supernet("aaa"),
        Sequence::Hypernet("kek"),
        Sequence::Supernet("eke"),
    ]));
    assert!(supports_ssl(&vec![
        Sequence::Supernet("zazbz"),
        Sequence::Hypernet("bzb"),
        Sequence::Supernet("cdb"),
    ]))
}

fn main() {
    let mut input = String::new();
    if let Ok(_) = File::open("input")
        .expect("cannot open input")
        .read_to_string(&mut input)
    {
        let mut tls_supported = 0;
        let mut ssl_supported = 0;

        for line in input.lines() {
            let sequences = parse_sequences(line);
            if supports_tls(&sequences) {
                tls_supported += 1;
            }
            if supports_ssl(&sequences) {
                ssl_supported += 1;
            }
        }
        println!("TLS: {}", tls_supported);
        println!("SSL: {}", ssl_supported);
    }
}
