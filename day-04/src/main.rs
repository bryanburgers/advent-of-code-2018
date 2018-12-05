use std::io::{Read, self};
use std::collections::HashMap;
use std::cmp::max;

extern crate regex;

mod input;
use input::TaggedInput;

mod sleep_range;
use sleep_range::SleepRange;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    let mut input : Vec<TaggedInput> = buffer.split("\n")
        .map(|i| i.trim())
        .map(|i| TaggedInput::parse(i).unwrap())
        .collect();

    input.sort();

    let ranges = SleepRange::get_ranges(input);

    let mut guards = HashMap::new();
    for range in ranges {
        let mut guard_info = guards.entry(range.guard).or_insert(GuardInfo::new());
        for minute in max(0, range.start_minute)..range.end_minute {
            guard_info.inc(minute);
        }
    }

    let (chosen_id, info) = guards.iter().max_by_key(|(_id, info)| info.total_asleep).unwrap();
    let chosen_minute = info.most_frequent_minute();

    println!("{}", chosen_id * chosen_minute);

    let (chosen_id, chosen_minute, _frequency) = guards.iter()
        .map(|(id, info)| {
            let minute = info.most_frequent_minute();
            let frequency = info.minutes_asleep.get(&minute).unwrap();
            (id, minute, frequency)
        })
        .max_by_key(|(_id, _minute, frequency)| *frequency)
        .unwrap();

    println!("{}", chosen_id * chosen_minute);
}

#[derive(Debug)]
struct GuardInfo {
    total_asleep: usize,
    minutes_asleep: HashMap<usize, usize>,
}

impl GuardInfo {
    fn new() -> GuardInfo {
        GuardInfo {
            total_asleep: 0,
            minutes_asleep: HashMap::new(),
        }
    }

    fn inc(&mut self, minute: usize) {
        self.total_asleep += 1;
        let x = self.minutes_asleep.entry(minute).or_insert(0);
        *x += 1;
    }

    fn most_frequent_minute(&self) -> usize {
        *self.minutes_asleep.iter().max_by_key(|(_minute, count)| *count).unwrap().0
    }
}
