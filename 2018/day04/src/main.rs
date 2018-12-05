use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

struct Guard {
    id: u16,
    schedule: [u8; 60],
    nap_start: u8,
}

impl Guard {
    fn new(id: u16) -> Guard {
        Guard {
            id,
            schedule: [0; 60],
            nap_start: 0,
        }
    }

    fn sleep(&mut self, minute: u8) {
        self.nap_start = minute;
    }

    fn awake(&mut self, minute: u8) {
        debug_assert!(minute > self.nap_start);
        self.schedule[self.nap_start as usize..minute as usize]
            .iter_mut()
            .for_each(|x| *x += 1);
    }

    fn sleepiest_minute(&self) -> Option<u8> {
        self.schedule
            .iter()
            .enumerate()
            .filter(|&(_, minute)| *minute > 0)
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(i, _)| i as u8)
    }

    fn minutes_slept(&self) -> u16 {
        self.schedule.iter().fold(0, |acc, &i| acc + u16::from(i))
    }
}

fn load_guards() -> io::Result<Vec<Guard>> {
    let mut buffer = String::new();
    File::open("input")?.read_to_string(&mut buffer)?;
    let mut lines: Vec<&str> = buffer.lines().collect();
    lines.sort_unstable();
    Ok(process_guards(lines))
}

fn process_guards(lines: Vec<&str>) -> Vec<Guard> {
    let mut guards = HashMap::new();
    let mut current_guard = u16::min_value();
    for line in lines {
        let (timestamp, message) = line.split_at(19);
        let minute = timestamp[15..17].parse().expect("cannot parse timestamp");
        if message.starts_with("Guard #") {
            let id: u16 = message
                .trim_matches(|c| !char::is_numeric(c))
                .parse()
                .expect("cannot parse guard ID");
            assert!(id != u16::min_value());
            let guard = guards.entry(id).or_insert_with(|| Guard::new(id));
            current_guard = guard.id;
        } else if message == "falls asleep" {
            guards.entry(current_guard).and_modify(|g| g.sleep(minute));
        } else if message == "wakes up" {
            guards.entry(current_guard).and_modify(|g| g.awake(minute));
        }
    }
    guards.into_iter().map(|(_, guard)| guard).collect()
}

fn strategy_one(guards: &[Guard]) -> Option<(&Guard, u8)> {
    guards
        .iter()
        .max_by_key(|guard| guard.minutes_slept())
        .and_then(|guard| guard.sleepiest_minute().map(|minute| (guard, minute)))
}

fn strategy_two(guards: &[Guard]) -> Option<(&Guard, u8)> {
    guards
        .iter()
        .filter_map(|guard| guard.sleepiest_minute().map(|minute| (guard, minute)))
        .max_by_key(|(guard, minute)| guard.schedule[*minute as usize])
}

fn main() {
    let guards = load_guards().expect("cannot parse input");
    match strategy_one(&guards) {
        Some((guard, minute)) => println!(
            "Strategy 1: {} * {} = {}",
            guard.id,
            minute,
            guard.id * u16::from(minute)
        ),
        _ => eprintln!("Strategy 1 failed"),
    }
    match strategy_two(&guards) {
        Some((guard, minute)) => println!(
            "Strategy 2: {} * {} = {}",
            guard.id,
            minute,
            guard.id * u16::from(minute)
        ),
        _ => eprintln!("Strategy 2 failed"),
    }
}

#[cfg(test)]
mod tests {
    fn logs() -> Vec<&'static str> {
        vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ]
    }

    #[test]
    fn strategy_one() {
        let guards = crate::process_guards(logs());
        let (guard, minute) = crate::strategy_one(&guards).unwrap();
        assert_eq!(10, guard.id);
        assert_eq!(24, minute);
    }

    #[test]
    fn strategy_two() {
        let guards = crate::process_guards(logs());
        let (guard, minute) = crate::strategy_two(&guards).unwrap();
        assert_eq!(99, guard.id);
        assert_eq!(45, minute);
    }
}
