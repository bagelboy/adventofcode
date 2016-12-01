use std::fs::File;
use std::io;
use std::io::Read;

struct Point {
    x: i32,
    y: i32,
}

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Turn {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Instruction {
    turn: Turn,
    distance: i32,
}

fn distance(p1: &Point, p2: &Point) -> u32 {
    ((p1.x - p2.x).abs() as u32) + ((p1.y - p2.y).abs() as u32)
}

fn direction_after_turn(direction: Direction, turn: Turn) -> Direction {
    match direction {
        Direction::North => {
            if turn == Turn::Left {
                return Direction::West;
            }
            return Direction::East;
        }
        Direction::East => {
            if turn == Turn::Left {
                return Direction::North;
            }
            return Direction::South;

        }
        Direction::South => {
            if turn == Turn::Left {
                return Direction::East;
            }
            return Direction::West;
        }
        Direction::West => {
            if turn == Turn::Left {
                return Direction::South;
            }
            return Direction::North;
        }
    }
}

fn read_input() -> Result<String, io::Error> {
    let mut input = String::new();
    File::open("input")?.read_to_string(&mut input)?;
    Ok(input)
}

fn parse_instructions(input: String) -> Result<Vec<Instruction>, io::Error> {
    let directions = input.trim()
        .split(", ")
        .map(|d| {
            let values = d.split_at(1);
            Instruction {
                turn: match values.0 {
                    "L" => Turn::Left,
                    _ => Turn::Right,
                },
                distance: values.1.parse::<i32>().unwrap(),
            }
        })
        .collect();

    Ok(directions)
}

fn follow_directions(directions: &Vec<Instruction>,
                     starting_point: &Point,
                     starting_direction: Direction)
                     -> Point {
    let mut x = starting_point.x;
    let mut y = starting_point.y;
    let mut direction = starting_direction;

    for d in directions {
        let new_direction = direction_after_turn(direction, d.turn.clone());
        match new_direction {
            Direction::North => {
                y += d.distance;
            }
            Direction::East => {
                x += d.distance;
            }
            Direction::South => {
                y -= d.distance;
            }
            Direction::West => {
                x -= d.distance;
            }
        }
        direction = new_direction;
    }

    Point { x: x, y: y }
}

fn main() {
    let input = read_input().unwrap();
    let directions = parse_instructions(input).unwrap();

    let start = Point { x: 0, y: 0 };
    let end = follow_directions(&directions, &start, Direction::North);

    println!("{}", distance(&start, &end));
}

#[test]
fn test_distance() {
    let origin = Point { x: 0, y: 0 };
    assert_eq!(5, distance(&origin, &Point { x: 2, y: 3 }));
    assert_eq!(2, distance(&origin, &Point { x: 0, y: -2 }));
    assert_eq!(12, distance(&origin, &Point { x: 10, y: 2 }));
}

#[test]
fn test_read_directions() {
    assert_eq!(parse_instructions("R2, R2, R2".to_string()).unwrap(),
               vec![Instruction {
                        turn: Turn::Right,
                        distance: 2,
                    }; 3]);
    assert_eq!(parse_instructions("L32, R128".to_string()).unwrap(),
               vec![Instruction {
                        turn: Turn::Left,
                        distance: 32,
                    },
                    Instruction {
                        turn: Turn::Right,
                        distance: 128,
                    }]);
}

#[test]
fn test_follow_directions() {
    let origin = Point { x: 0, y: 0 };
    assert_eq!(5,
               distance(&follow_directions(&parse_instructions("R2, L3".to_string()).unwrap(),
                                           &origin,
                                           Direction::North),
                        &origin));
    assert_eq!(2,
               distance(&follow_directions(&parse_instructions("R2, R2, R2".to_string())
                                               .unwrap(),
                                           &origin,
                                           Direction::North),
                        &origin));
    assert_eq!(12,
               distance(&follow_directions(&parse_instructions("R5, L5, R5, R3".to_string())
                                               .unwrap(),
                                           &origin,
                                           Direction::North),
                        &origin));
}
