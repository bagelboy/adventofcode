use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn most_blocks(blocks: &[usize]) -> Option<usize> {
    let mut index = None;
    let mut max = 0;
    for (i, v) in blocks.iter().enumerate() {
        if *v > max {
            max = *v;
            index = Some(i);
        }
    }
    index
}

fn reallocate(mut blocks: Vec<usize>) -> (usize, usize) {
    let mut cycles = 0;
    let mut seen = HashMap::new();

    while !seen.contains_key(&blocks) {
        seen.insert(blocks.clone(), cycles);
        cycles += 1;
        if let Some(max) = most_blocks(&blocks) {
            let count = blocks[max];
            blocks[max] = 0;
            for i in (max + 1)..(max + 1) + count {
                let insert_index = i % blocks.len();
                blocks[insert_index] += 1;
            }
        }
    }
    (cycles, cycles - seen[&blocks])
}

#[test]
fn test_reallocate() {
    assert_eq!((5, 4), reallocate(vec![0, 2, 7, 0]));
}

fn main() {
    let mut input = String::new();
    if File::open("input")
        .expect("cannot open input")
        .read_to_string(&mut input)
        .is_ok()
    {
        let memory_blocks: Vec<usize> = input
            .split_whitespace()
            .map(|b| b.parse().unwrap())
            .collect();
        let (cycles, length) = reallocate(memory_blocks);
        println!("cycles: {}", cycles);
        println!("length: {}", length);
    }
}
