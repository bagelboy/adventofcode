use std::fs::File;
use std::io::Read;

fn solve_captcha(input: &[char]) -> u32 {
    input.iter().enumerate().fold(0, |s, (i, j)| {
        if i < input.len() && input[i] == input[(i + 1) % input.len()] {
            s + j.to_digit(10).expect("this should be a number")
        } else {
            s
        }
    })
}

fn solve_new_captcha(input: &[char]) -> u32 {
    input.iter().enumerate().fold(0, |s, (i, j)| {
        if i < input.len() && input[i] == input[(i + input.len() / 2) % input.len()] {
            s + j.to_digit(10).expect("this should be a number")
        } else {
            s
        }
    })
}

fn get_chars(input: &str) -> Vec<char> {
    input.chars().collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_captcha() {
        assert_eq!(3, ::solve_captcha(&::get_chars(&"1122")));
        assert_eq!(4, ::solve_captcha(&::get_chars(&"1111")));
        assert_eq!(0, ::solve_captcha(&::get_chars(&"1234")));
        assert_eq!(9, ::solve_captcha(&::get_chars(&"91212129")));
    }

    #[test]
    fn solve_new_captcha() {
        assert_eq!(6, ::solve_new_captcha(&::get_chars("1212")));
        assert_eq!(0, ::solve_new_captcha(&::get_chars("1221")));
        assert_eq!(4, ::solve_new_captcha(&::get_chars("123425")));
        assert_eq!(12, ::solve_new_captcha(&::get_chars("123123")));
        assert_eq!(4, ::solve_new_captcha(&::get_chars("12131415")));
    }
}

fn main() {
    let mut input = String::new();
    if File::open("input")
        .expect("cannot open input")
        .read_to_string(&mut input)
        .is_ok()
    {
        let chars = get_chars(input.trim_right());
        println!("captcha: {}", solve_captcha(&chars));
        println!("new captcha: {}", solve_new_captcha(&chars));
    }
}
