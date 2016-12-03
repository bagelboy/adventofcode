use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::Read;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

enum Heading {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    turn_direction: Direction,
    blocks: i32,
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Block {
    x: i32,
    y: i32,
}

fn read_input() -> Result<String, io::Error> {
    let mut input = String::new();
    File::open("input")?.read_to_string(&mut input)?;
    Ok(input)
}

fn parse_instructions(input: String) -> Result<Vec<Instruction>, String> {
    let instructions = input.trim()
        .split(", ")
        .map(|i| {
            let values = i.split_at(1);
            Instruction {
                turn_direction: match values.0 {
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!("invalid turn"),
                },
                blocks: values.1.parse::<i32>().expect("invalid number of blocks"),
            }
        })
        .collect();

    Ok(instructions)
}

fn find_distance(instructions: Vec<Instruction>) -> Result<i32, String> {
    let mut position = Block { x: 0, y: 0 };
    let mut heading = Heading::North;

    for i in instructions {
        heading = move_position(&mut position, &heading, &i);
    }

    Ok(position.x.abs() + position.y.abs())
}

fn find_revisited_distance(instructions: Vec<Instruction>) -> Option<i32> {
    let mut position = Block { x: 0, y: 0 };
    let mut heading = Heading::North;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for i in instructions {
        heading = match move_and_track(&mut position, &mut visited, &heading, &i) {
            Some(heading) => heading,
            None => break,
        }
    }

    if !visited.contains(&(position.x, position.y)) {
        return None;
    }
    Some(position.x.abs() + position.y.abs())
}

fn calculate_new_heading(current_heading: &Heading, turn_direction: &Direction) -> Heading {
    match (current_heading, turn_direction) {
        (&Heading::North, &Direction::Right) => Heading::East,
        (&Heading::East, &Direction::Right) => Heading::South,
        (&Heading::South, &Direction::Right) => Heading::West,
        (&Heading::West, &Direction::Right) => Heading::North,

        (&Heading::North, &Direction::Left) => Heading::West,
        (&Heading::East, &Direction::Left) => Heading::North,
        (&Heading::South, &Direction::Left) => Heading::East,
        (&Heading::West, &Direction::Left) => Heading::South,
    }
}

fn move_position(position: &mut Block,
                 current_heading: &Heading,
                 instruction: &Instruction)
                 -> Heading {
    let heading = calculate_new_heading(current_heading, &instruction.turn_direction);

    match heading {
        Heading::North => {
            position.y += instruction.blocks;
        }
        Heading::East => {
            position.x += instruction.blocks;
        }
        Heading::South => {
            position.y -= instruction.blocks;
        }
        Heading::West => {
            position.x -= instruction.blocks;
        }
    }

    heading
}

fn move_and_track(position: &mut Block,
                  visited: &mut HashSet<(i32, i32)>,
                  current_heading: &Heading,
                  instruction: &Instruction)
                  -> Option<Heading> {
    let heading = calculate_new_heading(current_heading, &instruction.turn_direction);

    if !visited.contains(&(position.x, position.y)) {
        visited.insert((position.x, position.y)); // hack to break out of nested loop
    }

    let block_count = instruction.blocks;
    for _ in 0..block_count {
        match heading {
            Heading::North => {
                position.y += 1;
            }
            Heading::East => {
                position.x += 1;
            }
            Heading::South => {
                position.y -= 1;
            }
            Heading::West => {
                position.x -= 1;
            }
        }
        if visited.contains(&(position.x, position.y)) {
            return None // hack to break out of nested loop
        }
        visited.insert((position.x, position.y));
    }

    Some(heading)
}

fn main() {
    // transform error reading file into string
    fn stringify(e: io::Error) -> String {
        format!("{}", e)
    }

    match read_input().map_err(stringify).and_then(parse_instructions).and_then(find_distance) {
        Ok(distance) => println!("distance = {}", distance),
        Err(_) => println!("error"),
    }

    match read_input()
        .map_err(stringify)
        .and_then(parse_instructions)
        .and_then(|i| find_revisited_distance(i).ok_or("no convergence".to_string())) {
        Ok(distance) => println!("distance = {}", distance),
        Err(e) => println!("error: {}", e),
    }
}

#[test]
fn test_find_distance() {
    assert_eq!(5,
               parse_instructions("R2, L3".to_string()).and_then(find_distance).unwrap());
    assert_eq!(12,
               parse_instructions("R5, L5, R5, R3".to_string()).and_then(find_distance).unwrap());
    assert_eq!(2,
               parse_instructions("R2, R2, R2".to_string()).and_then(find_distance).unwrap());
}

#[test]
fn test_first_visited_twice() {
    assert_eq!(4,
               parse_instructions("R8, R4, R4, R8".to_string())
                   .and_then(|i| find_revisited_distance(i).ok_or("error".to_string()))
                   .unwrap());
    assert_eq!(0,
               parse_instructions("L2, R2, R2, R3, L6".to_string())
                   .and_then(|i| find_revisited_distance(i).ok_or("error".to_string()))
                   .unwrap());
}
