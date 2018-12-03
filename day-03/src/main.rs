use std::io::{Read, self};
use std::collections::HashSet;

mod square;

use square::Square;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    let squares : Vec<Square> = buffer.split("\n")
        .map(|i| i.trim())
        .map(|i| Square::parse(i).unwrap())
        .collect();

    let mut overlaps = HashSet::new();

    for (idx, s1) in squares.iter().enumerate() {
        for s2 in &squares[idx+1..] {
            let intersection = s1.intersect(s2);
            if let Some(intersection) = intersection {
                for point in intersection.points() {
                    overlaps.insert(point);
                }
            }
        }
    }

    println!("{}", overlaps.len());
}
