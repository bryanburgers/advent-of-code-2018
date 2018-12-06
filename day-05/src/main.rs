use std::io::{Read, self};
use std::collections::VecDeque;


fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    let answer_1 = react_polymer(buffer.bytes());

    println!("{}", answer_1.len());

    let mut min = answer_1.len();
    for i in b'A'..=b'Z' {
        let new_iterator = buffer.bytes()
            .filter(|letter| *letter != i && *letter != (i + b'a' - b'A'));

        let reacted = react_polymer(new_iterator);
        if reacted.len() < min {
            min = reacted.len();
        }
    }
    println!("{}", min);
}

fn react_polymer(polymer: impl IntoIterator<Item=u8>) -> String {
    let mut stack = VecDeque::new();

    let mut current = 0;

    for byte in polymer {
        if react(byte, current) {
            current = stack.pop_back().unwrap_or(0);
        }
        else {
            if current > 0 {
                stack.push_back(current);
            }
            current = byte;
        }
    }

    stack.push_back(current);

    let x: Vec<u8> = stack.iter().map(|x| *x).collect();
    let s = std::str::from_utf8(x.as_ref()).unwrap();
    s.to_owned()
}

fn print_state(stack: &VecDeque<u8>, current: u8, read: u8) {
    let x: Vec<u8> = stack.iter().map(|x| *x).collect();
    let s = std::str::from_utf8(x.as_ref());

    if read == 0 {
        println!("{}-{}", s.unwrap(), current as char);
    }
    else {
        println!("{}-{}-{}", s.unwrap(), current as char, read as char);
    }
}

fn react(a: u8, b: u8) -> bool {
    let diff = b'a' - b'A';
    if a < b'A' || b < b'A' {
        return false;
    }

    if a >= b && a - diff == b {
        return true;
    }

    b - diff == a
}
