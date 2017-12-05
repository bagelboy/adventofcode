use std::fs::File;
use std::io::Read;

fn escape_jump_maze(offsets: &[i32]) -> u32 {
    let mut offsets_copy = offsets.to_vec();
    let mut position = 0;
    let mut steps = 0;
    loop {
        steps += 1;
        let new_position = position as i32 + offsets_copy[position];
        offsets_copy[position] += 1;
        if new_position < 0 || new_position >= offsets_copy.len() as i32 {
            break;
        }
        position = new_position as usize;
    }
    steps
}

#[test]
fn test_escape_jump_maze() {
    assert_eq!(5, escape_jump_maze(&vec![0, 3, 0, 1, -3]));
}

fn escape_strange_jump_maze(offsets: &[i32]) -> u32 {
    let mut offsets_copy = offsets.to_vec();
    let mut position = 0;
    let mut steps = 0;
    loop {
        steps += 1;
        let new_position = position as i32 + offsets_copy[position];
        if offsets_copy[position] < 3 {
            offsets_copy[position] += 1;
        } else {
            offsets_copy[position] -= 1;
        }
        if new_position < 0 || new_position >= offsets_copy.len() as i32 {
            break;
        }
        position = new_position as usize;
    }
    steps
}

#[test]
fn test_escape_strange_jump_maze() {
    assert_eq!(10, escape_strange_jump_maze(&vec![0, 3, 0, 1, -3]));
}

fn main() {
    let mut input = String::new();
    if File::open("input")
        .expect("cannot read input")
        .read_to_string(&mut input)
        .is_ok()
    {
        let offsets: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();

        println!("maze one: {} steps", escape_jump_maze(&offsets));
        println!("maze two: {} steps", escape_strange_jump_maze(&offsets));
    }
}
