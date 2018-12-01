use std::io::{Read, self};

fn main() {
    println!("Hello, world!");
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    let result = buffer.split("\n")
        .map(|i| i.trim())
        .map(|i| i.parse::<i32>().unwrap())
        .fold(0, |acc, i| acc + i);

    println!("{}", result);
}
