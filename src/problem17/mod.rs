fn get_target(line: String) -> ((i32, i32), (i32, i32)) {
    let line = line.as_str()[13..line.len()].to_string();
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

fn gauss(n: i32) -> f32 {
    ((n * (n + 1)) / 2) as f32
}

pub fn p17_1(data: &Vec<String>) -> i32 {
    let (_, y_range) = get_target(data[0].clone());
    gauss(-(y_range.0 + 1)) as i32
}

fn step(coord: i32, is_x: bool) -> Vec<(i32, i32, bool)> {
    let mut steps = Vec::new();
    for initial_vel in (0..(coord + 1)).rev() {
        let mut step = 0;
        let mut remaining_vel = initial_vel;
        let mut sum = remaining_vel;
        while sum <= coord && remaining_vel >= 0 {
            if sum == coord {
                let step = if is_x {step} else {(initial_vel * 2) + step};
                steps.push((step, initial_vel, remaining_vel > 0));
            }
            remaining_vel += if is_x {-1} else {1};
            sum += remaining_vel;
            step += 1;
        }
    }
    steps
}

pub fn p17_2(data: &Vec<String>) -> i32 {
    let (x_range, y_range) = get_target(data[0].clone());

    let mut steps_y = Vec::new();
    let y_range = ((y_range.1.abs())..(y_range.0.abs() + 1)).collect::<Vec<i32>>();
    for y in &y_range {
        steps_y.append(&mut step(*y, false));
    }

    let mut steps_x = Vec::new();
    let x_range = ((x_range.0.abs())..(x_range.1.abs() + 1)).collect::<Vec<i32>>();
    for x in &x_range {
        steps_x.append(&mut step(*x, true));
    }

    steps_x.sort();
    steps_y.sort();

    let initial_len = steps_y.len();
    for i in 0..initial_len {
        if steps_y[i].1 != 0 {
            steps_y.push((steps_y[i].0 - ((steps_y[i].1) * 2), -steps_y[i].1, steps_y[i].2));
        }
    }

    steps_y.sort();
    steps_y.pop();

    let mut couples = Vec::new();
    for x in &steps_x {
        for y in &steps_y {
            if y.0 == x.0 {
                couples.push((x, y));
            }
            if x.2 == false && x.0 <= y.0 {
                couples.push((x, y));
            }
        }
    }

    let mut dedup_couples = Vec::<((i32, i32, bool), (i32, i32, bool))>::new();
    for c in couples {
        if !dedup_couples.iter().any(|c_| c.0.1 == c_.0.1 && c.1.1 == c_.1.1) {
            let c = (*c.0, *c.1);
            dedup_couples.push(c);
        }
    }

    dedup_couples.len() as i32
}

