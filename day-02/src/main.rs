use std::io::{Read, self};
use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    let input : Vec<&str> = buffer.split("\n")
        .map(|i| i.trim())
        .collect();

    let twos = input.iter()
        .filter(|i| has_exactly(i, 2))
        .count();

    let threes = input.iter()
        .filter(|i| has_exactly(i, 3))
        .count();

    println!("{} * {} = {}", twos, threes, twos * threes);
}

fn has_exactly(v: &str, n: usize) -> bool {
    let r = v.chars().fold(HashMap::new(), |mut acc, val| {
        {
            let e = acc.entry(val).or_insert(0);
            *e += 1;
        }

        acc
    });

    r.values().any(|x| *x == n)
}
