use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn traverse_firewall(layers: &HashMap<u32, u32>, delay: u32) -> (u32, u32) {
    let mut penalty = 0;
    let mut caught = 0;

    for (ticks, range) in layers {
        // [0 1 2, ..., r-2, r-1, r-2, ..., 2, 1]
        if (ticks + delay) % ((range - 1) * 2) == 0 {
            penalty += ticks * range;
            caught += 1;
        }
    }
    (penalty, caught)
}

fn find_delay(layers: &HashMap<u32, u32>) -> u32 {
    let mut delay = 0;
    while traverse_firewall(layers, delay).1 > 0 {
        delay += 1;
    }
    delay
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    fn get_test_input() -> HashMap<u32, u32> {
        let mut layers = HashMap::new();
        layers.insert(0, 3);
        layers.insert(1, 2);
        layers.insert(4, 4);
        layers.insert(6, 4);
        layers
    }

    #[test]
    fn test_traverse_firewall() {
        assert_eq!((24, 2), ::traverse_firewall(&get_test_input(), 0));
    }

    #[test]
    fn test_find_delay() {
        assert_eq!(10, ::find_delay(&get_test_input()));
    }
}

fn main() {
    let file = File::open("input").expect("cannot open input");
    let file = BufReader::new(file);

    let mut layers = HashMap::new();
    for line in file.lines() {
        let line = line.expect("cannot read line");
        let mut values = line.split(": ");
        let depth = values.next().expect("cannot read depth").parse().unwrap();
        let range = values.next().expect("cannot read range").parse().unwrap();
        layers.insert(depth, range);
    }
    println!("severity: {}", traverse_firewall(&layers, 0).0);
    println!("delay: {}", find_delay(&layers));
}
