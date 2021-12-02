pub fn p1_1(data: &Vec<String>) -> u32 {
    let mut prev: Option<i32> = None;
    let mut count: u32 = 0;

    for string in data {
        let current: i32 = string.parse().expect("Not a valid input value");
        if prev.is_some() && prev.unwrap() < current {
            count += 1;
        }
        prev = Some(current);
    }

    count
}

pub fn p1_2(data: &Vec<String>) -> u32 {
    let mut prev: u32 = 0;
    let mut count: u32 = 0;
    let data_num: Vec<u32> = data
        .iter()
        .map(|string| string.parse::<u32>().expect("Not a valid input value"))
        .collect();

    for i in 1..(data_num.len() - 1) {
        let current = data_num[i-1] + data_num[i] + data_num[i+1];
        if i > 1 && current > prev {
            count += 1;
        }
        prev = current; 
    }

    count
}

