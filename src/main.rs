use std::io;
use std::env;
use std::io::{BufRead};

mod problem1;
mod problem2;
use problem1::{p1_1, p1_2};
use problem2::{p2_1, p2_2};

pub fn input() -> Vec<String> {
    let stdin = io::stdin();
    let mut data = stdin.lock().lines();
    let mut lines = Vec::<String>::new();
    while let Some(line) = data.next() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        lines.push(line);
    }
    lines
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let pno: u32 = args[1].parse().expect("Arg value can't be converted to u32");

    match pno {
        1 => {
            let input = input();
            println!("p1: {} p2: {}", p1_1(&input), p1_2(&input));
        },
        2 => {
            let input = input();
            println!("p1: {} p2: {}", p2_1(&input), p2_2(&input));
        },
        _ => (),
    }
}

