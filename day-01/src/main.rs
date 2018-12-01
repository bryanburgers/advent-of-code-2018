use std::io::{Read, self};
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    let input : Vec<i32> = buffer.split("\n")
        .map(|i| i.trim())
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    let result_a = input.iter().fold(0, |acc, i| acc + i);
    println!("{}", result_a);

    let mut frequency = 0;
    let mut frequencies = HashSet::new();
    'outer: loop {
        for i in &input {
            frequency += *i;
            if frequencies.contains(&frequency) {
                break 'outer;
            }
            frequencies.insert(frequency);
        }
    }

    println!("{}", frequency);
}
