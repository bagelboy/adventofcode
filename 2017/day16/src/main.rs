use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;
use std::str;

fn spin(programs: &mut Vec<char>, size: usize) {
    for _ in 0..size {
        let foo = programs.pop().unwrap();
        programs.insert(0, foo);
    }
}

#[test]
fn test_spin() {
    let mut programs = vec!['a', 'b', 'c', 'd', 'e'];
    spin(&mut programs, 1);
    assert_eq!(vec!['e', 'a', 'b', 'c', 'd'], programs);
}

fn exchange(programs: &mut [char], a: usize, b: usize) {
    programs.swap(a, b);
}

#[test]
fn test_exchange() {
    let mut programs = vec!['e', 'a', 'b', 'c', 'd'];
    exchange(&mut programs, 3, 4);
    assert_eq!(vec!['e', 'a', 'b', 'd', 'c'], programs);
}

fn partner(programs: &mut [char], a: char, b: char) {
    let i = programs.iter().position(|&c| c == a).unwrap();
    let j = programs.iter().position(|&c| c == b).unwrap();
    programs.swap(i, j);
}

#[test]
fn test_partner() {
    let mut programs = vec!['e', 'a', 'b', 'd', 'c'];
    partner(&mut programs, 'e', 'b');
    assert_eq!(vec!['b', 'a', 'e', 'd', 'c'], programs);
}

fn dance(programs: &mut Vec<char>, dance_move: &str) {
    match dance_move.chars().nth(0).unwrap() {
        's' => {
            let amount: usize = dance_move[1..].parse().unwrap();
            spin(programs, amount);
        }
        'x' => {
            let mut amounts = dance_move[1..].split('/');
            let a = amounts.next().unwrap().parse().unwrap();
            let b = amounts.next().unwrap().parse().unwrap();
            exchange(programs, a, b);
        }
        'p' => {
            let mut amounts = dance_move[1..].split('/');
            let a = amounts.next().unwrap().chars().nth(0).unwrap();
            let b = amounts.next().unwrap().chars().nth(0).unwrap();
            partner(programs, a, b);
        }
        _ => panic!("bad dance move!"),
    }
}

#[test]
fn test_dance() {
    let mut programs = vec!['a', 'b', 'c', 'd', 'e'];
    dance(&mut programs, "s1");
    dance(&mut programs, "x3/4");
    dance(&mut programs, "pe/b");
    assert_eq!(vec!['b', 'a', 'e', 'd', 'c'], programs);
}

fn go_dancing(programs: &mut Vec<char>, dance_moves: &[&str], count: usize) -> String {
    let mut seen: Vec<Vec<char>> = Vec::new();
    for _ in 0..count {
        if seen.contains(&programs) {
            return String::from_iter(seen[count % seen.len()].iter());
        }
        seen.push(programs.clone());
        for m in dance_moves {
            dance(programs, m);
        }
    }

    String::from_iter(programs.iter())
}

#[test]
fn test_go_dancing() {
    let mut programs = vec!['a', 'b', 'c', 'd', 'e'];
    let dance_moves = vec!["s1", "x3/4", "pe/b"];
    assert_eq!("baedc", go_dancing(&mut programs, &dance_moves, 1));
    assert_eq!("ceadb", go_dancing(&mut programs, &dance_moves, 1));
}

fn main() {
    let mut programs = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];

    let mut input = String::new();
    File::open("input")
        .expect("cannot open input")
        .read_to_string(&mut input)
        .expect("cannot read input");

    let moves: Vec<&str> = input.trim_right().split(',').collect();

    println!("  single dance: {}", go_dancing(&mut programs, &moves, 1));
    println!("billion dances: {}", go_dancing(&mut programs, &moves, 1000000000 - 1));
}
