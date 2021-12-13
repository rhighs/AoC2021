use std::io;
use std::env;
use std::io::{BufRead};

mod problem1;
mod problem2;
mod problem3;
mod problem4;
mod problem5;
mod problem6;
mod problem7;
mod problem8;
mod problem9;
mod problem10;
mod problem11;
mod problem12;
mod problem13;
use problem1::{p1_1, p1_2};
use problem2::{p2_1, p2_2};
use problem3::{p3_1, p3_2};
use problem4::{p4_1, p4_2};
use problem5::{p5_1, p5_2};
use problem6::{p6_1, p6_2};
use problem7::{p7_1, p7_2};
use problem8::{p8_1, p8_2};
use problem9::{p9_1, p9_2};
use problem10::{p10_1, p10_2};
use problem11::{p11_1, p11_2};
use problem12::{p12_1, p12_2};
use problem13::{p13_1, p13_2};

//Accepts input via stdin, EOF needed
pub fn input() -> Vec<String> {
    let stdin = io::stdin();
    let mut data = stdin.lock().lines();
    let mut lines = Vec::<String>::new();
    while let Some(line) = data.next() {
        if !line.is_ok() {
            break;
        }
        let line = line.unwrap();
        lines.push(line);
    }
    lines
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let pno: u32 = args[1].parse().expect("Arg value can't be converted to u32");

    let input = input();

    match pno {
        1 => println!("p1: {} p2: {}", p1_1(&input), p1_2(&input)),
        2 => println!("p1: {} p2: {}", p2_1(&input), p2_2(&input)),
        3 => println!("p1: {} p2: {}", p3_1(&input), p3_2(&input)),
        4 => println!("p1: {} p2: {}", p4_1(&input), p4_2(&input)),
        5 => println!("p1: {} p2: {}", p5_1(&input), p5_2(&input)),
        6 => println!("p1: {} p2: {}", p6_1(&input), p6_2(&input)),
        7 => println!("p1: {} p2: {}", p7_1(&input), p7_2(&input)),
        8 => println!("p1: {} p2: {}", p8_1(&input), p8_2(&input)),
        9 => println!("p1: {} p2: {}", p9_1(&input), p9_2(&input)),
        10 => println!("p1: {} p2: {}", p10_1(&input), p10_2(&input)),
        11 => println!("p1: {} p2: {}", p11_1(&input), p11_2(&input)),
        12 => println!("p1: {} p2: {}", p12_1(&input), p12_2(&input)),
        13 => println!("p1: {} p2: {}", p13_1(&input), p13_2(&input)),
        _ => (),
    }
}

