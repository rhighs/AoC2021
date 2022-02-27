fn positions(data: &Vec<String>) -> (u32, u32) {
    let parsed: Vec<u32> = data.iter()
        .map(|string| string.split(":").last().unwrap())
        .map(|cut_value| cut_value.trim_start())
        .map(|string| string.parse::<u32>().unwrap())
        .collect();
    (parsed[0], parsed[1])
}

fn movepawn(roll: u32, curr_pos: u32) -> u32 {
    let pos = (roll + curr_pos) % 10;
    if pos == 0 {10} else {pos}
}

pub fn p21_1(data: &Vec<String>) -> u32 {
    let mut rolls = 0;
    let (mut p1_score, mut p2_score) = (0, 0);
    let (mut p1_pos, mut p2_pos) = positions(data);

    let mut dice = 1u32..;
    let mut roll = || dice.by_ref().take(3).sum();
    let mut p1_turn = true;

    let p1_wins = loop {
        let rollvalue = roll();

        if p1_turn {
            p1_pos = movepawn(rollvalue, p1_pos);
            p1_score += p1_pos;
        } else {
            p2_pos = movepawn(rollvalue, p2_pos);
            p2_score += p2_pos;
        }

        let p1_wins = p1_score >= 1000;
        let p2_wins = p2_score >= 1000;

        p1_turn = !p1_turn;
        rolls += 3;

        if p1_wins || p2_wins {
            break p1_wins;
        }
    };

    let loser_score = if p1_wins {p2_score} else {p1_score};
    loser_score * rolls
}

pub fn p21_2(data: &Vec<String>) -> u32 {
    0
}
