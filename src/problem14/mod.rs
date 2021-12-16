use std::collections::HashMap;

const TOKEN: &str = " -> ";

fn rules(data: &Vec<String>) -> HashMap<(char, char), char> {
    let mut rules = HashMap::new();
    for string in data {
        if string.contains(TOKEN) {
            let splitted = string.split(TOKEN).collect::<Vec<&str>>();
            let pair_1 = splitted[0].as_bytes()[0] as char;
            let pair_2 = splitted[0].as_bytes()[1] as char;
            let end = splitted[1].as_bytes()[0] as char;
            rules.insert((pair_1, pair_2), end);
        }
    }
    rules
}

fn solve(data: &Vec<String>, steps: usize) -> u64 {
    let polymer = data[0].trim_end().to_string();
    let rules = rules(data);
    let mut pairs = HashMap::<(char, char), u64>::new();

    for i in 0..polymer.len() - 1 {
        let pair_1 = polymer.as_bytes()[i] as char;
        let pair_2 = polymer.as_bytes()[i+1] as char;
        pairs.entry((pair_1, pair_2))
            .and_modify(|value| *value += 1)
            .or_insert(1);
    }

    let mut chars_counts: HashMap<char, u64> = HashMap::new();
    for c in polymer.chars() {
        chars_counts.entry(c).and_modify(|val| *val += 1).or_insert(1);
    }

    for _ in 0..steps {
        let mut new_pairs = pairs.clone();

        for pair in &pairs {
            let target = rules.get(pair.0);

            if target.is_some() {
                let target = target.unwrap();
                let new_pair_1 = (pair.0.0, *target);
                let new_pair_2 = (*target, pair.0.1);

                if *pair.1 == 0 {
                    continue;
                }

                chars_counts
                    .entry(*target)
                    .and_modify(|val| *val += *pair.1)
                    .or_insert(*pair.1);

                new_pairs.entry(*pair.0)
                    .and_modify(|value| *value -= *pair.1);
                new_pairs.entry(new_pair_1)
                    .and_modify(|value| *value += *pair.1)
                    .or_insert(*pair.1);
                new_pairs.entry(new_pair_2)
                    .and_modify(|value| *value += *pair.1)
                    .or_insert(*pair.1);
            }
        }

        pairs = new_pairs;
    }

    chars_counts.values().max().unwrap() - chars_counts.values().min().unwrap() 
}

pub fn p14_1(data: &Vec<String>) -> u64 {
    solve(data, 10)
}

pub fn p14_2(data: &Vec<String>) -> u64 {
    solve(data, 40)
}

