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
    let y_coord = split_coord[1][2..split_coord[1].len()]
        .to_string()
        .split("..")
        .collect::<Vec<&str>>()
        .iter()
        .map(|value| value.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    ((x_coord[0], x_coord[1]), (y_coord[0], y_coord[1]))
}

fn inv_gauss(m: i32) -> bool {
    let n = ((2 * m) as f32 + 0.25f32).sqrt() - 0.5f32;
    n.fract() == 0.0f32
}

pub fn p17_1(data: &Vec<String>) -> i32 {
    let (x_range, y_range) = get_target(data[0].clone());
    let mut x_speed = 0;
    let y_speed = (y_range.0 + 1) * -1;

    for i in x_range.0..x_range.1 {
        if inv_gauss(i) {
            x_speed = i;
        }
    }

    println!("Starting velocity: {:?}", (x_speed, y_speed));
    (0..y_speed + 1).collect::<Vec<_>>().iter().sum()
}

pub fn p17_2(data: &Vec<String>) -> (i32, i32) {
    (0, 0)
}

