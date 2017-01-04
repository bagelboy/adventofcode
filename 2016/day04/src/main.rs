use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn get_valid_room_id(room: &str) -> Option<u32> {
    let last_dash = room.rfind('-').expect("missing '-' in data");
    let (letters, the_rest) = room.split_at(last_dash);
    let actual_checksum = find_checksum(letters.replace('-', ""));

    let open_brace = the_rest.find('[').expect("missing '[' in data");
    let room_id = the_rest[1..open_brace].parse().expect("cannot parse room id");
    let expected_checksum = &the_rest[open_brace + 1..the_rest.len() - 1];

    if expected_checksum != actual_checksum {
        return None;
    }
    return Some(room_id);
}

fn get_rotated_room(room: &str) -> Option<u32> {
    let last_dash = room.rfind('-').expect("missing '-' in data");
    let (letters, the_rest) = room.split_at(last_dash);

    let open_brace = the_rest.find('[').expect("missing '[' in data");
    let room_id = the_rest[1..open_brace].parse().expect("cannot parse room id");

    if rotate_string(letters, room_id).starts_with("north") {
        return Some(room_id);
    }
    None
}

fn find_checksum(letters: String) -> String {
    let mut counts: HashMap<char, u32> = HashMap::new();
    for l in letters.chars() {
        let count = counts.entry(l).or_insert(0);
        *count += 1;
    }

    let mut checksum = String::new();
    while !counts.is_empty() && checksum.len() < 5 {
        let mut candidates: Vec<char> = Vec::new();
        let max = *(counts.values().max().expect("cannot find max"));
        for (letter, count) in &counts {
            if *count == max {
                candidates.push(*letter);
            }
        }
        candidates.sort();
        for l in candidates {
            if checksum.len() < 5 {
                checksum.push(l);
            }
            counts.remove(&l);
        }
    }
    checksum
}

fn rotate_string(original: &str, shift_by: u32) -> String {
    static START: u8 = 'a' as u8;
    static END: u8 = 'z' as u8;

    original.chars()
        .map(|c| match c {
            'a'...'z' => {
                let mut shifted = c as u8 + (shift_by % 26) as u8;
                if shifted > END {
                    shifted = (shifted % END) + (START - 1);
                }
                shifted as char
            }
            '-' => ' ',
            _ => c,
        })
        .collect()
}

#[test]
fn rotate_string_test() {
    assert_eq!(String::from("very encrypted name"),
               rotate_string(&"qzmt-zixmtkozy-ivhz", 343));
    assert_eq!(String::from("a"), rotate_string(&"z", 1));
}

fn main() {
    let sum: u32 = BufReader::new(File::open("input").expect("cannot open input"))
        .lines()
        .map(|line| get_valid_room_id(&line.expect("cannot read line")).unwrap_or(0))
        .sum();
    println!("sum of sector IDs of real rooms: {}", sum);

    for line in BufReader::new(File::open("input").expect("cannot open input")).lines() {
        match get_rotated_room(&line.expect("cannot read line")) {
            Some(room_id) => {
                println!("north pole objects in sector {}", room_id);
                break;
            }
            _ => continue,
        }
    }
}

#[test]
fn get_valid_room_id_test() {
    assert_eq!(Some(123), get_valid_room_id(&"aaaaa-bbb-z-y-x-123[abxyz]"));
    assert_eq!(Some(987), get_valid_room_id(&"a-b-c-d-e-f-g-h-987[abcde]"));
    assert_eq!(Some(404), get_valid_room_id(&"not-a-real-room-404[oarel]"));
    assert_eq!(None, get_valid_room_id(&"totally-real-room-200[decoy]"));
}
