fn main() {
    let steps = 394;

    // part one
    {
        use std::collections::VecDeque;

        let iterations = 2017;

        let mut buffer = VecDeque::with_capacity(iterations + 1);
        buffer.push_back(0);
        let mut current_position = 0;
        for i in 1..(iterations + 1) {
            let new_position = (current_position + steps) % i + 1;
            buffer.insert(new_position, i);
            current_position = new_position;
        }

        let index = buffer.iter().position(|&v| v == iterations).unwrap() + 1;
        println!("value after {}: {}", iterations, buffer[index]);
    }

    // part two
    {
        let mut last_inserted = 0;
        let mut current_position = 0;

        for i in 1..(50_000_001) {
            let new_position = (current_position + steps) % i;
            if new_position == 0 {
                last_inserted = i;
            }
            current_position = new_position + 1;
        }

        println!("value after 0: {}", last_inserted);
    }
}
