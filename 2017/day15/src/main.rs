use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static FACTOR_A: u64 = 16807;
static FACTOR_B: u64 = 48271;
static DIVISOR: u64 = 2147483647;

struct Generator {
    value: u64,
    factor: u64,
    discriminator: u64,
}

impl Generator {
    pub fn new(seed: u64, factor: u64, discriminator: u64) -> Generator {
        Generator {
            value: seed,
            factor,
            discriminator,
        }
    }

    pub fn next(&mut self) -> u64 {
        let new_value = (self.value * self.factor) % DIVISOR;
        self.value = new_value;
        self.value
    }

    pub fn next_picky(&mut self) -> u64 {
        let mut value = self.value;
        loop {
            let new_value = (value * self.factor) % DIVISOR;
            if new_value % self.discriminator == 0 {
                self.value = new_value;
                return self.value;
            }
            value = new_value;
        }
    }
}

#[test]
fn test_generator() {
    {
        let mut a = Generator::new(65, FACTOR_A, 4);
        assert_eq!(1092455, a.next());
        assert_eq!(1181022009, a.next());
        assert_eq!(245556042, a.next());
        assert_eq!(1744312007, a.next());
        assert_eq!(1352636452, a.next());
    }

    {
        let mut b = Generator::new(8921, FACTOR_B, 8);
        assert_eq!(430625591, b.next());
        assert_eq!(1233683848, b.next());
        assert_eq!(1431495498, b.next());
        assert_eq!(137874439, b.next());
        assert_eq!(285222916, b.next());
    }
}

#[test]
fn test_picky() {
    {
        let mut a = Generator::new(65, FACTOR_A, 4);
        assert_eq!(1352636452, a.next_picky());
        assert_eq!(1992081072, a.next_picky());
        assert_eq!(530830436, a.next_picky());
        assert_eq!(1980017072, a.next_picky());
        assert_eq!(740335192, a.next_picky());
    }

    {
        let mut b = Generator::new(8921, FACTOR_B, 8);
        assert_eq!(1233683848, b.next_picky());
        assert_eq!(862516352, b.next_picky());
        assert_eq!(1159784568, b.next_picky());
        assert_eq!(1616057672, b.next_picky());
        assert_eq!(412269392, b.next_picky());
    }
}

#[test]
fn test_matches() {
    let gen_a = Generator::new(65, FACTOR_A, 4);
    let gen_b = Generator::new(8921, FACTOR_B, 8);

    assert_eq!(1, run_generators(gen_a, gen_b, 5));
}

#[test]
fn test_matches_picky() {
    let gen_a = Generator::new(65, FACTOR_A, 4);
    let gen_b = Generator::new(8921, FACTOR_B, 8);

    assert_eq!(1, run_generators_picky(gen_a, gen_b, 1056));
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

fn run_generators_picky(mut gen_a: Generator, mut gen_b: Generator, iterations: usize) -> u32 {
    let mut matches = 0;
    let mask = std::u16::MAX as u64;
    for _ in 0..iterations {
        let (a, b) = (gen_a.next_picky(), gen_b.next_picky());

        if (a & mask) == (b & mask) {
            matches += 1;
        }
    }
    matches
}

fn main() {
    let file = File::open("input").expect("cannot open input");
    let mut lines = BufReader::new(file).lines();
    let a = get_input_value(&lines.next().unwrap().unwrap());
    let b = get_input_value(&lines.next().unwrap().unwrap());

    println!(
        "count: {}",
        run_generators(
            Generator::new(a, FACTOR_A, 4),
            Generator::new(b, FACTOR_B, 8),
            40_000_000
        )
    );
    println!(
        "new logic count: {}",
        run_generators_picky(
            Generator::new(a, FACTOR_A, 4),
            Generator::new(b, FACTOR_B, 8),
            5_000_000
        )
    );
}
