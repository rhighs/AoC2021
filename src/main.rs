use std::env;

mod problem1;
use problem1::{p1_input, p1_1, p1_2};

fn main() {
    let args: Vec<String> = env::args().collect();
    let pno: u32 = args[1].parse().expect("Arg value can't be converted to u32");

    match pno {
        1 => {
            let input = p1_input();
            println!("p1: {} p2: {}", p1_1(&input), p1_2(&input));
        },
        _ => (),
    }
}

