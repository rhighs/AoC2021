fn days(states: Vec<u32>, target_day: usize) -> u64 {
    let mut offsets = vec![0usize; 10]; //10th zero to allow a reset by shift
    for state in states { offsets[state as usize] += 1; }
    for _ in 0..target_day {
        let zeros: usize = offsets[0];
        for i in 1..offsets.len() {
            offsets[i - 1] = offsets[i];
        }
        offsets[6] += zeros;
        offsets[8] += zeros;
    }
    offsets.iter().sum::<usize>() as u64
}

pub fn p6_1(data: &Vec<String>) -> u64 {
    days(data[0]
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<Vec<u32>>(), 80)
}

pub fn p6_2(data: &Vec<String>) -> u64 {
    days(data[0]
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<Vec<u32>>(), 256)
}

