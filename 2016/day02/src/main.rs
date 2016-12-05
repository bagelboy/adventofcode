use std::fs::File;
use std::io;
use std::io::Read;

fn read_input() -> Result<String, io::Error> {
    let mut input = String::new();
    File::open("input")?.read_to_string(&mut input)?;
    Ok(input)
}

fn get_combination(start_at: u32,
                   dimension: u32,
                   instructions: String)
                   -> Result<String, &'static str> {
    let mut position = start_at;
    let mut combination = String::new();

    for line in instructions.lines() {
        for instr in line.chars() {
            match instr {
                'U' => {
                    if position > dimension {
                        position -= dimension;
                    }
                }
                'D' => {
                    if position <= (dimension * dimension - dimension) {
                        position += dimension;
                    }
                }
                'L' => {
                    if position % dimension != 1 {
                        position -= 1;
                    }
                }
                'R' => {
                    if position % dimension > 0 {
                        position += 1;
                    }
                }
                _ => return Err("invalid instruction in input!"),
            }
        }
        combination.push(std::char::from_digit(position, 10).unwrap());
    }

    Ok(combination)
}

fn main() {
    let dimension = 3;

    let input = read_input().unwrap();
    match get_combination(5, dimension, input) {
        Ok(combination) => println!("{}", combination),
        Err(e) => println!("oh noes! {}", e),
    }
}

#[test]
fn test_instructions() {
    let combination = get_combination(5, 3, "ULL\nRRDDD\nLURDL\nUUUUD".to_string()).unwrap();

    assert_eq!("1985", combination);
}
