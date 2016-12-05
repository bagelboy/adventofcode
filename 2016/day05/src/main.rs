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

fn main() {
    println!("{}", find_password("ffykfhsq"));
}

#[test]
fn test_find_password() {
    // run `cargo test --release` or this takes a LONG time!
    assert_eq!("18f47a30", find_password("abc"));
}
