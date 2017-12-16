use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
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

fn main() {
    let mut programs = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];

    let file = File::open("input").expect("cannot open input");
    let mut file = BufReader::new(file);

    let mut buffer: Vec<u8> = Vec::new();
    while file
        .read_until(b',', &mut buffer)
        .expect("error reading input")
        > 0
    {
        {
            let dance_move = str::from_utf8(&buffer)
                .unwrap()
                .trim_right_matches(|c| c == '\n' || c == ',');
            dance(&mut programs, dance_move);
        }
        buffer.clear();
    }
    let output = String::from_iter(programs);
    println!("{}", output);
}
