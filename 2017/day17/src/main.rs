use std::collections::VecDeque;

fn main() {
    let steps = 394;

    let mut buffer = VecDeque::with_capacity(2018);
    buffer.push_back(0);
    let mut current_position = 0;
    for i in 0..2017 {
        let new_position = (current_position + steps) % buffer.len() + 1;
        buffer.insert(new_position, i + 1);
        current_position = new_position;
    }

    let index = buffer.iter().position(|&v| v == 2017).unwrap() + 1;
    println!("value after 2017: {}", buffer[index]);
}
