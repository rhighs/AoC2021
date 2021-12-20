fn get_target(line: String) -> ((i32, i32), (i32, i32)) {
    let line = line.as_str()[13..line.len() - 1].to_string();
    let split_coord = line.split(" ").collect::<Vec<&str>>();
    let x_coord = split_coord[0][2..split_coord[0].len() - 1]
        .to_string()
        .split("..")
        .collect::<Vec<&str>>()
        .iter()
        .map(|value| value.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let y_coord = split_coord[1][2..split_coord[0].len() - 1]
        .to_string()
        .split("..")
        .collect::<Vec<&str>>()
        .iter()
        .map(|value| value.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    ((x_coord[0], x_coord[1]), (y_coord[0], y_coord[1]))
}

pub fn p17_1(data: &Vec<String>) -> u32 {
    println!("{:?}", get_target(data[0]));
    0
}

pub fn p17_2(data: &Vec<String>) -> u32 {
    0
}

