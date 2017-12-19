use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

enum Value {
    Constant(i64),
    Register(char),
}

enum Instruction {
    Snd(char),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jgz(Value, Value),
}

fn parse_value(argument: Option<&str>) -> Value {
    if argument.unwrap().chars().all(|c| c.is_alphabetic()) {
        Value::Register(argument.unwrap().chars().nth(0).unwrap())
    } else {
        Value::Constant(argument.unwrap().parse().expect("this should be a number"))
    }
}

fn parse_register(argument: Option<&str>) -> char {
    argument
        .expect("need a value")
        .chars()
        .nth(0)
        .expect("length should be > 0")
}

fn parse_instruction(instruction: &str) -> Instruction {
    let mut parts = instruction.split_whitespace();
    let name = parts.next().expect("this should be an instruction");
    let destination = parts.next();
    match name {
        "snd" => Instruction::Snd(parse_register(destination)),
        "set" => Instruction::Set(parse_register(destination), parse_value(parts.next())),
        "add" => Instruction::Add(parse_register(destination), parse_value(parts.next())),
        "mul" => Instruction::Mul(parse_register(destination), parse_value(parts.next())),
        "mod" => Instruction::Mod(parse_register(destination), parse_value(parts.next())),
        "rcv" => Instruction::Rcv(parse_register(destination)),
        "jgz" => Instruction::Jgz(parse_value(destination), parse_value(parts.next())),
        _ => panic!("Invalid instruction: {}", name),
    }
}

fn execute_instructions(instructions: &[Instruction]) -> i64 {
    use std::collections::HashMap;

    let mut last_frequency = 0;
    let mut program_counter = 0;
    let mut registers = HashMap::new();

    while program_counter >= 0 && program_counter < instructions.len() as i64 {
        let instruction = &instructions[program_counter as usize];
        match *instruction {
            Instruction::Snd(src) => {
                let value = registers.entry(src).or_insert(0);
                last_frequency = *value;
            }
            Instruction::Set(dst, ref v) => match *v {
                Value::Constant(c) => {
                    registers.insert(dst, c);
                }
                Value::Register(src) => {
                    let val = *registers.entry(src).or_insert(0);
                    registers.insert(dst, val);
                }
            },
            Instruction::Add(dst, ref v) => match *v {
                Value::Constant(c) => {
                    let entry = registers.entry(dst).or_insert(0);
                    *entry += c;
                }
                Value::Register(src) => {
                    let src = *registers.entry(src).or_insert(0);
                    let dest = registers.entry(dst).or_insert(0);
                    *dest += src;
                }
            },
            Instruction::Mul(dst, ref v) => match *v {
                Value::Constant(c) => {
                    let entry = registers.entry(dst).or_insert(0);
                    *entry *= c;
                }
                Value::Register(src) => {
                    let src = *registers.entry(src).or_insert(0);
                    let dest = registers.entry(dst).or_insert(0);
                    *dest *= src;
                }
            },
            Instruction::Mod(dst, ref v) => match *v {
                Value::Constant(c) => {
                    let entry = registers.entry(dst).or_insert(0);
                    *entry %= c;
                }
                Value::Register(src) => {
                    let src = *registers.entry(src).or_insert(0);
                    let dest = registers.entry(dst).or_insert(0);
                    *dest %= src;
                }
            },
            Instruction::Rcv(r) => {
                let value = *registers.entry(r).or_insert(0);
                // part one
                if value != 0 {
                    return last_frequency;
                }
            }
            Instruction::Jgz(ref valx, ref valy) => {
                let x = match *valx {
                    Value::Register(r) => *registers.entry(r).or_insert(0),
                    Value::Constant(c) => c,
                };
                if x > 0 {
                    let y = match *valy {
                        Value::Constant(c) => c,
                        Value::Register(r) => *registers.entry(r).or_insert(0),
                    };
                    program_counter += y;
                    continue;
                }
            }
        }
        program_counter += 1;
    }

    last_frequency
}

#[test]
fn test_execute() {
    let input = vec![
        "set a 1", "add a 2", "mul a a", "mod a 5", "snd a", "set a 0", "rcv a", "jgz a -1",
        "set a 1", "jgz a -2",
    ];
    let instructions: Vec<Instruction> = input.iter().map(|i| parse_instruction(i)).collect();

    assert_eq!(4, execute_instructions(&instructions));
}

fn main() {
    let reader = BufReader::new(File::open("input").expect("cannot open input"));

    let instructions: Vec<Instruction> = reader
        .lines()
        .map(|l| parse_instruction(&l.unwrap()))
        .collect();
    println!(
        "recovered frequency: {}",
        execute_instructions(&instructions)
    );
}
