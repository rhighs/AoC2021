use std::str::FromStr;
use std::io::{BufRead};
use std::io;

pub fn p2_input() -> Vec<String> {
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

pub fn p2_1(data: &Vec<String>) -> i32 {
    let mut d: i32 = 0;
    let mut h: i32 = 0;
    for string in data {
        let mut split = string.split(" ");
        let command: &str = split.next().unwrap();
        let value: i32 = FromStr::from_str(split.next().unwrap()).unwrap();
        match command {
            "forward"=> h += value,
            "up" => d -= value,
            "down" => d += value,
            _ => ()
        }
    }
    d * h
}

