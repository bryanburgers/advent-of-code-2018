use std::io::{Read, self};
use std::collections::VecDeque;


fn main() {
    let mut stack = VecDeque::new();

    let mut current = 0;

    for byte in io::stdin().bytes() {
        let mut byte = byte.expect("Input shouldn't be None");
        if byte < b'A' {
            continue;
        }

        if react(byte, current) {
            current = stack.pop_back().expect("Expected stack to contain values");
        }
        else {
            stack.push_back(current);
            current = byte;
        }
    }

    println!("{}", stack.len());
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
