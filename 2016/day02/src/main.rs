use std::fs::File;
use std::io;
use std::io::Read;

#[cfg_attr(rustfmt, rustfmt_skip)]
static SQUARE_KEYPAD: [[u32; 3]; 3] = [[1, 2, 3],
                                       [4, 5, 6],
                                       [7, 8, 9]];

#[cfg_attr(rustfmt, rustfmt_skip)]
static DIAMOND_KEYPAD: [[char; 5]; 5] = [['0', '0', '1', '0', '0'],
                                        ['0', '2', '3', '4', '0'],
                                        ['5', '6', '7', '8', '9'],
                                        ['0', 'A', 'B', 'C', '0'],
                                        ['0', '0', 'D', '0', '0']];

fn read_input() -> Result<String, io::Error> {
    let mut input = String::new();
    File::open("input")?.read_to_string(&mut input)?;
    Ok(input)
}

fn get_combination(instructions: &String) -> Result<String, &'static str> {
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

fn get_diamond_combination(instructions: &String) -> Result<String, &'static str> {
    let mut x = 0;
    let mut y = 2;
    let mut combination = String::new();

    for line in instructions.lines() {
        for instr in line.chars() {
            match instr {
                'U' => {
                    if y > 0 && DIAMOND_KEYPAD[y - 1][x] != '0' {
                        y -= 1;
                    }
                }
                'D' => {
                    if y < DIAMOND_KEYPAD.len() - 1 && DIAMOND_KEYPAD[y + 1][x] != '0' {
                        y += 1;
                    }
                }
                'L' => {
                    if x > 0 && DIAMOND_KEYPAD[y][x - 1] != '0' {
                        x -= 1;
                    }
                }
                'R' => {
                    if x < DIAMOND_KEYPAD.len() - 1 && DIAMOND_KEYPAD[y][x + 1] != '0' {
                        x += 1;
                    }
                }
                _ => return Err("invalid instruction in input!"),
            }
        }
        combination.push(DIAMOND_KEYPAD[y][x]);
    }

    Ok(combination)
}

fn print_combination(result: Result<String, &'static str>) {
    match result {
        Ok(combination) => println!("{}", combination),
        Err(e) => println!("Error! {}", e),
    }
}

fn main() {
    let input = read_input().unwrap();

    print!("First combination:  ");
    print_combination(get_combination(&input));

    print!("{}", "Second combination: ");
    print_combination(get_diamond_combination(&input));
}

#[test]
fn test_instructions() {
    let instructions = "ULL\nRRDDD\nLURDL\nUUUUD".to_owned();

    assert_eq!("1985", get_combination(&instructions).unwrap());
    assert_eq!("5DB3", get_diamond_combination(&instructions).unwrap());
}
