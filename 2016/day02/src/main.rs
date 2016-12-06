use std::fs::File;
use std::io;
use std::io::Read;

#[cfg_attr(rustfmt, rustfmt_skip)]
static SQUARE_KEYPAD: [[u32; 3]; 3] = [[1, 2, 3],
                                       [4, 5, 6],
                                       [7, 8, 9]];

fn read_input() -> Result<String, io::Error> {
    let mut input = String::new();
    File::open("input")?.read_to_string(&mut input)?;
    Ok(input)
}

fn get_combination(instructions: String) -> Result<String, &'static str> {
    let mut combination = String::new();
    let mut x = 1;
    let mut y = 1;

    for line in instructions.lines() {
        for instr in line.chars() {
            match instr {
                'U' => {
                    if y > 0 {
                        y -= 1;
                    }
                }
                'D' => {
                    if y < SQUARE_KEYPAD.len() - 1 {
                        y += 1;
                    }
                }
                'L' => {
                    if x > 0 {
                        x -= 1;
                    }
                }
                'R' => {
                    if x < SQUARE_KEYPAD.len() - 1 {
                        x += 1;
                    }
                }
                _ => return Err("invalid instruction in input!"),
            }
        }
        combination.push(std::char::from_digit(SQUARE_KEYPAD[y][x], 10).unwrap());
    }

    Ok(combination)
}

fn main() {
    let input = read_input().unwrap();
    match get_combination(input) {
        Ok(combination) => println!("{}", combination),
        Err(e) => println!("oh noes! {}", e),
    }
}

#[test]
fn test_instructions() {
    let combination = get_combination(5, 3, "ULL\nRRDDD\nLURDL\nUUUUD".to_string()).unwrap();

    assert_eq!("1985", combination);
}
