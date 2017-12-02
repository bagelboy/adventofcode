use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn parse_numbers(raw_input: &str, data: &mut Vec<u32>) {
    data.clear();
    data.extend(
        raw_input
            .split('\t')
            .map(|number| u32::from_str(number).unwrap()),
    );
}

fn calculate_checksum(numbers: &[u32]) -> Option<u32> {
    if let (Some(max), Some(min)) = (numbers.iter().max(), numbers.iter().min()) {
        Some(max - min)
    } else {
        None
    }
}

fn calculate_even_checksum(numbers: &[u32]) -> Option<u32> {
    for x in numbers {
        for y in numbers {
            if x == y {
                continue;
            }
            if x % y == 0 {
                return Some(x / y);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn checksum() {
        assert_eq!(8, ::calculate_checksum(&vec![5, 1, 9, 5]).unwrap());
        assert_eq!(4, ::calculate_checksum(&vec![7, 5, 3]).unwrap());
        assert_eq!(6, ::calculate_checksum(&vec![2, 4, 6, 8]).unwrap());
    }

    #[test]
    fn even_checksum() {
        assert_eq!(4, ::calculate_even_checksum(&vec![5, 9, 2, 8]).unwrap());
        assert_eq!(3, ::calculate_even_checksum(&vec![9, 4, 7, 3]).unwrap());
        assert_eq!(2, ::calculate_even_checksum(&vec![3, 8, 6, 5]).unwrap());
    }
}

fn main() {
    let mut input = String::new();
    let mut numbers: Vec<u32> = Vec::new();

    if File::open("input")
        .expect("cannot open input")
        .read_to_string(&mut input)
        .is_ok()
    {
        let mut first_checksum = 0;
        let mut second_checksum = 0;
        for line in input.lines() {
            parse_numbers(line, &mut numbers);
            if let Some(value) = calculate_checksum(&numbers) {
                first_checksum += value;
            }
            if let Some(value) = calculate_even_checksum(&numbers) {
                second_checksum += value;
            }
        }
        println!("{}", first_checksum);
        println!("{}", second_checksum);
    }
}
