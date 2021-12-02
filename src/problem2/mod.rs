use std::str::FromStr;

fn parse_command(commandline: &String) -> (String, i32) {
    let mut split = commandline.split(" ");
    let command: &str = split.next().unwrap();
    let value: i32 = FromStr::from_str(split.next().unwrap()).unwrap();
    (String::from(command), value)
}

pub fn p2_1(data: &Vec<String>) -> i32 {
    let mut d: i32 = 0;
    let mut h: i32 = 0;
    for string in data {
        let (command, value) = parse_command(string);
        match &command[..] {
            "forward"=> h += value,
            "up" => d -= value,
            "down" => d += value,
            _ => ()
        }
    }
    d * h
}

pub fn p2_2(data: &Vec<String>) -> i32 {
    let mut aim: i32 = 0;
    let mut h: i32 = 0;
    let mut d: i32 = 0;
    for string in data {
        let (command, value) = parse_command(string);
        match &command[..] {
            "forward"=> { h += value; d += aim * value; },
            "up" => aim -= value,
            "down" => aim += value,
            _ => ()
        }
    }
    d * h
}

