use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

enum ResultType {
    MAX,
    MIN,
}

fn correct_message(messages: &Vec<&str>, result_type: ResultType) -> String {
    let length = messages[0].len();
    let mut frequencies: Vec<HashMap<char, u32>> = Vec::with_capacity(length);
    for _ in 0..length {
        frequencies.push(HashMap::new());
    }

    for m in messages {
        for (i, c) in m.chars().enumerate() {
            let count = frequencies[i].entry(c).or_insert(0);
            *count += 1;
        }
    }

    frequencies.into_iter()
        .map(|counts| {
            let needle = match result_type {
                ResultType::MAX => counts.values().max().unwrap(),
                ResultType::MIN => counts.values().min().unwrap(),
            };
            for key in counts.keys() {
                if counts[key] == *needle {
                    return *key;
                }
            }
            panic!("Cannot find key matching value");
        })
        .collect()
}

#[test]
fn correct_message_test() {
    let messages = vec!["eedadn", "drvtee", "eandsr", "raavrd", "atevrs", "tsrnev", "sdttsa",
                        "rasrtv", "nssdts", "ntnada", "svetve", "tesnvt", "vntsnd", "vrdear",
                        "dvrsen", "enarar"];
    assert_eq!(correct_message(&messages, ResultType::MAX),
               String::from("easter"));
    assert_eq!(correct_message(&messages, ResultType::MIN),
               String::from("advent"));
}

fn main() {
    let mut input = String::new();
    if File::open("input")
        .expect("cannot open input")
        .read_to_string(&mut input)
        .is_ok() {
        let messages: Vec<_> = input.split_whitespace().collect();
        println!("{}", correct_message(&messages, ResultType::MAX));
        println!("{}", correct_message(&messages, ResultType::MIN));
    }
}
