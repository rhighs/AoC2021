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

fn diff(first_digit: &String, second_digit: &String) -> String {
    first_digit.chars().filter(|&c| !second_digit.contains(c)).collect::<String>()
}

pub fn p8_2(data: &Vec<String>) -> u32 {
    let p = parse_mappings(data);
    let mappings = p.0;
    let codes = p.1;
    let mut sum = 0;

    let mut mappings = mappings
        .chunks(10)
        .map(|codes| codes.into())
        .collect::<Vec<Vec<String>>>();

    let codes = codes
        .chunks(4)
        .map(|codes| codes.into())
        .collect::<Vec<Vec<String>>>();

    for i in 0..mappings.len() {
        let mapping = &mut mappings[i];

        mapping.sort_by(|a, b| a.len().cmp(&b.len()));

        let mut parsed_numbers: [String; 10] = [
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ];

        let mut uniques = Vec::new();
        let mut sixes = Vec::new();
        let mut fives = Vec::new();
        for connection in mapping.clone() {
            match connection.len() {
                2 => uniques.push(connection),
                3 => uniques.push(connection),
                4 => uniques.push(connection),
                5 => fives.push(connection),
                6 => sixes.push(connection),
                7 => uniques.push(connection),
                _ => ()
            };
        }

        let one = &uniques[0];
        let seven = & uniques[1];
        let four = &uniques[2];
        let eight = &uniques[3];
        parsed_numbers[1] = one.clone();
        parsed_numbers[4] = four.clone();
        parsed_numbers[7] = seven.clone();
        parsed_numbers[8] = eight.clone();

        let mut one_upper = String::new();

        for segment in sixes {
            let difference = diff(one, &segment);
            let mut was_6 = false;
            if difference.len() == 1 {
                one_upper = difference;
                parsed_numbers[6] = segment.clone();
                was_6 = true;
            }

            let difference_eight = diff(&eight, &segment);
            let difference_four = diff(&four, &segment);
            if difference_eight.len() == 1 && difference_four.len() == 0 {
                parsed_numbers[9] = segment.clone();
            } else if !was_6 {
                parsed_numbers[0] = segment.clone();
            }
        }

        for segment in fives {
            if diff(&one_upper, &segment).len() == 0 && diff(&one, &segment).len() == 1 {
                parsed_numbers[2] = segment.clone();
            } else if diff(&one, &segment).len() == 0 {
                parsed_numbers[3] = segment.clone();
            } else {
                parsed_numbers[5] = segment.clone();
            }
        }

        let mut number_as_string = String::new();
        for code in &codes[i] {
            for (i, n) in parsed_numbers.iter().enumerate() {
                if n.len() == code.len() && diff(n, code).len() == 0 {
                    number_as_string += &i.to_string();
                }
            }
        }

        sum += number_as_string.parse::<u32>().unwrap();
    }

    sum
}

