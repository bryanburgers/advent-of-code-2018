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

    let mut best = 0;
    for i in input.clone().iter() {
        for j in input.clone().iter() {
            if i == j {
                continue;
            }

            let res = compare(i, j);
            let val = res.len();
            if val > best {
                best = val;

                println!("{}, {} => {} ({})", i, j, val, res);
            }
        }
    }
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

fn compare(v1: &str, v2: &str) -> String {
    let i1 = v1.chars();
    let i2 = v2.chars();
    let matches: String = i1.zip(i2)
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect();

    matches
}
