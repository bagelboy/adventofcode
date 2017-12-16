use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static FACTOR_A: u64 = 16807;
static FACTOR_B: u64 = 48271;
static DIVISOR: u64 = 2147483647;

struct Generator {
    value: u64,
    factor: u64,
}

impl Generator {
    pub fn new(seed: u64, factor: u64) -> Generator {
        Generator {
            value: seed,
            factor,
        }
    }

    pub fn next(&mut self) -> u64 {
        let new_value = (self.value * self.factor) % DIVISOR;
        self.value = new_value;
        new_value
    }
}

#[test]
fn test_generator() {
    {
        let mut a = Generator::new(65, FACTOR_A);
        assert_eq!(1092455, a.next());
        assert_eq!(1181022009, a.next());
        assert_eq!(245556042, a.next());
        assert_eq!(1744312007, a.next());
        assert_eq!(1352636452, a.next());
    }

    {
        let mut b = Generator::new(8921, FACTOR_B);
        assert_eq!(430625591, b.next());
        assert_eq!(1233683848, b.next());
        assert_eq!(1431495498, b.next());
        assert_eq!(137874439, b.next());
        assert_eq!(285222916, b.next());
    }
}

#[test]
fn test_matches() {
    let mut gen_a = Generator::new(65, FACTOR_A);
    let mut gen_b = Generator::new(8921, FACTOR_B);

    assert_eq!(1, run_generators(gen_a, gen_b, 5));
}

fn get_input_value(line: &str) -> u64 {
    let parts = line.split_whitespace();
    parts
        .last()
        .expect("invalid input")
        .parse()
        .expect("this should be a number")
}

fn run_generators(mut gen_a: Generator, mut gen_b: Generator, iterations: usize) -> u32 {
    let mut matches = 0;
    let mask = std::u16::MAX as u64;
    for _ in 0..iterations {
        let (a, b) = (gen_a.next(), gen_b.next());

        if (a & mask) == (b & mask) {
            matches += 1;
        }
    }
    matches
}

fn main() {
    let file = File::open("input").expect("cannot open input");
    let mut lines = BufReader::new(file).lines();
    let gen_a = Generator::new(get_input_value(&lines.next().unwrap().unwrap()), FACTOR_A);
    let gen_b = Generator::new(get_input_value(&lines.next().unwrap().unwrap()), FACTOR_B);

    println!("count: {}", run_generators(gen_a, gen_b, 40_000_000));
}
