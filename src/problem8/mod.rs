fn parse_mappings(data: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let parse = |idx: usize| data
        .iter()
        .map(|string| string.split(" | ").collect::<Vec<&str>>()[idx])
        .flat_map(|string| string.split(" ").collect::<Vec<&str>>())
        .map(String::from)
        .collect();
    (parse(0), parse(1))
}

pub fn p8_1(data: &Vec<String>) -> u32 {
    let codes = parse_mappings(data).1;
    let mut sum = 0;

    let numbers: [Vec<usize>; 10] = [
        vec![0, 1, 2, 4, 5, 6], //0
        vec![2, 5],             //1
        vec![0, 2, 3, 4, 6],    //2
        vec![0, 2, 3, 5, 6],    //3
        vec![1, 2, 3, 5],       //4
        vec![0, 1, 3, 5, 6],    //5
        vec![0, 1, 3, 4, 5, 6], //6
        vec![0, 2, 5],          //7
        vec![0, 1, 2, 3, 4, 5, 6], //8
        vec![0, 1, 2, 3, 4, 6]     //9
    ];

    for code in codes {
        let maybe = numbers
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, num)| num.len() == code.len())
            .collect::<Vec<(usize, Vec<usize>)>>();
        sum += maybe
            .iter()
            .filter(|(idx, _)| *idx == 1 || *idx == 4 || *idx == 7 || *idx == 8)
            .count() as u32;
    }

    sum
}

pub fn p8_2(data: &Vec<String>) -> u32 {
    0
}

