fn crab_positions(data: &Vec<String>) -> Vec<i32> {
    data[0]
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&string| string.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

pub fn p7_1(data: &Vec<String>) -> i32 {
    let mut values = crab_positions(data);
    values.sort();
    let m: i32 = if values.len() % 2 == 0 {(values[(values.len() / 2)] + values[(values.len() / 2) - 1]) / 2} else {values[(values.len() / 2) - 1]};
    values.iter().map(|&value| (value - m).abs()).sum()
}

pub fn p7_2(data: &Vec<String>) -> i32 {
    let values = crab_positions(data);
    let m = (values.iter().sum::<i32>() as f32 / values.len() as f32).round() as i32;
    values
        .iter()
        .map(|&value|(1..((value - m).abs() + 1))
                      .collect::<Vec<i32>>()
                      .iter()
                      .sum::<i32>())
        .sum::<i32>()
}

