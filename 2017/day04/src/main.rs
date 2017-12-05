use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;

fn is_valid_passphrase(passphrase: &str) -> bool {
    let words: Vec<&str> = passphrase.split_whitespace().collect();
    let all_words = words.len();
    let unique_words = HashSet::<&str>::from_iter(words).len();
    all_words == unique_words
}

fn is_valid_anagram_passphrase(passphrase: &str) -> bool {
    let words: Vec<&str> = passphrase.split_whitespace().collect();
    let all_words = words.len();
    let unique_words = words
        .into_iter()
        .map(|word| {
            let mut letters: Vec<char> = word.chars().collect();
            letters.sort_unstable();
            String::from_iter(letters)
        })
        .collect::<HashSet<String>>()
        .len();
    all_words == unique_words
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_valid_passphrase() {
        assert!(::is_valid_passphrase(&"aa bb cc dd"));
        assert!(!::is_valid_passphrase(&"aa bb cc dd aa"));
        assert!(::is_valid_passphrase(&"aa bb cc dd aaa"));
    }

    #[test]
    fn is_valid_anagram_passphrase() {
        assert!(::is_valid_anagram_passphrase(&"abcde fghij"));
        assert!(!::is_valid_anagram_passphrase(&"abcde xyz ecdab"));
        assert!(::is_valid_anagram_passphrase(&"a ab abc abd abf abj"));
        assert!(::is_valid_anagram_passphrase(&"iiii oiii ooii oooi oooo"));
        assert!(!::is_valid_anagram_passphrase(&"oiii ioii iioi iiio"));
    }
}

fn main() {
    let mut input = String::new();
    if File::open("input")
        .expect("cannot open input")
        .read_to_string(&mut input)
        .is_ok()
    {
        let mut valid_passphrases = 0;
        let mut valid_anagram_passphrases = 0;

        for line in input.lines() {
            if is_valid_passphrase(line) {
                valid_passphrases += 1;
            }
            if is_valid_anagram_passphrase(line) {
                valid_anagram_passphrases += 1;
            }
        }

        println!("valid passphrases: {}", valid_passphrases);
        println!("valid anagram passphrases: {}", valid_anagram_passphrases);
    }
}
