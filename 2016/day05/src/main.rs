extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn find_password(input: &str) -> String {
    let mut hasher = Md5::new();
    let mut password = String::new();

    for i in 0.. {
        hasher.reset();
        hasher.input_str(input);
        hasher.input_str(&i.to_string());
        let result = hasher.result_str();
        if result.starts_with("00000") {
            password.push(result.chars().nth(5).unwrap());
            if password.len() == 8 {
                break;
            }
        }
    }
    password
}

fn find_second_password(input: &str) -> String {
    let mut hasher = Md5::new();
    let mut password = ['_'; 8];
    let mut found = 0;

    for i in 0.. {
        hasher.reset();
        hasher.input_str(input);
        hasher.input_str(&i.to_string());
        let result = hasher.result_str();
        if result.starts_with("00000") {
            let mut chars = result.chars();
            let position = chars.nth(5).unwrap();
            let character = chars.next().unwrap();
            if let Some(index) = position.to_digit(10) {
                let actual_index = index as usize;
                if index < 8 && password[actual_index] == '_' {
                    password[actual_index] = character;
                    found += 1;
                }
            }
            if found == 8 {
                break;
            }
        }
    }

    password.iter().cloned().collect::<String>()
}

fn main() {
    println!("first door:  {}", find_password("ffykfhsq"));
    println!("second door: {}", find_second_password("ffykfhsq"));
}

#[test]
fn test_find_password() {
    // run `cargo test --release` or this takes a LONG time!
    assert_eq!("18f47a30", find_password("abc"));
    assert_eq!("05ace8e3", find_second_password("abc"));
}
