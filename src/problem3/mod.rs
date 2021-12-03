fn common(data: &Vec<String>, most_less: bool) -> Vec<(bool, u32)> {
    let mut values = Vec::new();
    for i in 0..data[0].len() {
        values.push(0);
        for string in data {
            let string_str = string.to_string();
            let bytes = string_str.as_bytes();
            if (bytes[i] as char == '1') == most_less {
                values[i] += 1;
            }
        }
    }
    values
        .iter()
        .map(|&value| (value > ((data.len() / 2) as u32), value))
        .collect()
}

fn convert(bits: Vec<(bool, u32)>) -> u32 {
    let mut value = 0;
    for (i, &bit) in bits.iter().enumerate() {
        value += if bit.0 == true {2u32.pow((bits.len() - i - 1) as u32)} else {0};
    }
    value
}

pub fn p3_1(data: &Vec<String>) -> u32 {
    let bits = common(data, true);
    let flipbits = common(data, false);
    convert(bits) * convert(flipbits)
}

pub fn choose_number(data: Vec<String>, step: usize, most_less: bool) -> Vec<String> {
    if data.len() < 2 {
        return data;
    }
    let mut chosen = Vec::new();
    let mut common = common(&data, most_less)[step];
    if common.1 == (data.len() / 2) as u32 && data.len() % 2 == 0 {
        common.0 = most_less;
    }
    for string in data {
        if common.0 == (string.as_bytes()[step] as char == '1') {
            chosen.push(string);
        }
    }
    choose_number(chosen, step + 1, most_less)
}
 
pub fn p3_2(data: &Vec<String>) -> u32 {
    let oxigen_data = data.clone();
    let carb_data = data.clone();
    let oxigen = choose_number(oxigen_data, 0, true);
    let carb = choose_number(carb_data, 0, true);
    convert(common(&oxigen, true)) * convert(common(&carb, true))
}

