const OPEN_PAREN: i128 = -1;
const CLOSED_PAREN: i128 = -11;

fn add(n1: &Vec<i128>, n2: &Vec<i128>) -> Vec<i128> {
    let mut number = Vec::new();
    number.push(OPEN_PAREN);
    number.append(&mut n1.clone());
    number.append(&mut n2.clone());
    number.push(CLOSED_PAREN);
    number
}

fn convert(n: &String) -> Vec<i128> {
    n.chars()
        .filter(|&c| c != ',')
        .map(|c|
            match c {
            '[' => OPEN_PAREN,
            ']' => CLOSED_PAREN,
            _ => c.to_string().parse::<i128>().unwrap()
            }
        ).collect::<Vec<i128>>()
}

fn check(number: &Vec<i128>, start_pos: usize, curr_pos: usize, paren_count: u32) -> (bool, Option<usize>){
    let mut explode_at = (false, None);
    if number[start_pos] != OPEN_PAREN {
        return explode_at;
    }
    if curr_pos < number.len() {
        if number[curr_pos] == OPEN_PAREN {
            if paren_count == 4 {
                return (true, Some(curr_pos));
            }
            explode_at = check(number, start_pos, curr_pos + 1, paren_count + 1);
        } else if number[curr_pos] == CLOSED_PAREN {
            if paren_count == 1 {
                return explode_at;
            }
            return check(number, start_pos, curr_pos + 1, paren_count - 1);
        } else {
            return check(number, start_pos, curr_pos + 1, paren_count);
        }
    }
    return explode_at;
}

fn explode(number: &Vec<i128>) -> (Vec<i128>, bool) {
    let mut converted = number.clone();
    let mut starting_pos = 0;

    let find_pair = |number: &Vec<i128>, mut starting_pos: usize| -> usize {
        while number[starting_pos + 1] < 0 || number[starting_pos + 2] < 0 {
            starting_pos += 1;
        }
        starting_pos
    };

    let explode_at = check(&converted, starting_pos, starting_pos, 0);
    loop {
        if explode_at.0 {
            let found_pos = explode_at.1.unwrap();
            let pair_pos = find_pair(&converted, found_pos);
            let n1 = converted[pair_pos + 1];
            let n2 = converted[pair_pos + 2];
            for _ in 0..4 {
                converted.remove(pair_pos);
            }
            converted.insert(pair_pos, 0);

            let mut pos = pair_pos - 1;
            loop {
                if converted[pos] != OPEN_PAREN && converted[pos] != CLOSED_PAREN {
                    converted[pos] += n1;
                    break;
                }
                if pos == 0 {
                    break;
                }
                pos -= 1;
            }

            let mut pos = pair_pos + 1;
            loop {
                if converted[pos] != OPEN_PAREN && converted[pos] != CLOSED_PAREN {
                    converted[pos] += n2;
                    break;
                }
                if pos == converted.len() - 1 {
                    break;
                }
                pos += 1;
            }

            return (converted, true);
        } else {
            starting_pos += 1;
        }

        if starting_pos == converted.len() {
            break;
        }
    }
    (converted, false)
}

fn split(number: &Vec<i128>) -> (Vec<i128>, bool) {
    let mut converted = number.clone();
    let mut pos = 0;
    let result = loop {
        if converted[pos] >= 10 {
            let n1 = converted[pos] / 2;
            let n2 = (converted[pos] + 1) / 2;
            converted.remove(pos);
            converted.insert(pos, CLOSED_PAREN);
            converted.insert(pos, n2);
            converted.insert(pos, n1);
            converted.insert(pos, OPEN_PAREN);
            return (converted, true);
        }
        pos += 1;
        if pos == converted.len() {
            break (converted, false);
        }
    };
    return result;
}

fn reduce(number: Vec<i128>) -> Vec<i128> {
    let (c1, has_exploded) = explode(&number);
    if has_exploded {
        return reduce(c1);
    } else {
        let (c2, has_splitted) = split(&number);
        if has_splitted {
            return reduce(c2);
        } else {
            return c2;
        }
    }
}

fn magnitude(number: &Vec<i128>) -> i128 {
    let mut number = number.clone();
    while number.len() != 4 {
        let mut pos = 0;
        while pos < number.len() - 2 {
            if number[pos + 1] >= 0 && number[pos + 2] >= 0 {
                let m = (3 * number[pos + 1]) + (2 * number[pos + 2]);
                for _ in 0..4 {
                    number.remove(pos);
                }
                number.insert(pos, m);
            }
            pos += 1;
        }
    }
    number.remove(0);
    number.pop();
    (number[0] * 3) + (number[1] * 2)
}

pub fn p18_1(data: &Vec<String>) -> i128 {
    let mut sum = convert(&data[0]);
    let numbers = data
        .iter()
        .map(|value| convert(value))
        .collect::<Vec<Vec<i128>>>();
    for number in &numbers {
        let addition = add(&sum, &number);
        sum = reduce(addition);
    }
    magnitude(&sum)
}

pub fn p18_2(data: &Vec<String>) -> i128 {
    let mut mgs = Vec::new();
    let numbers = data
        .iter()
        .map(|value| convert(value))
        .collect::<Vec<Vec<i128>>>();
    for (i, n1) in numbers.iter().enumerate() {
        for (j, n2) in numbers.iter().enumerate() {
            if i != j {
                let sum = &reduce(add(n1, n2));
                mgs.push(magnitude(sum));
            }
        }
    }
    mgs.into_iter().max().unwrap()
}

