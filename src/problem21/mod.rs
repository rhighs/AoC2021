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

const FREQ: [u128; 7] = [1, 3, 6, 7, 6, 3, 1];

#[derive(Clone, Copy)]
struct Universe {
    p1_pos: u32,
    p2_pos: u32,
    p1_score: u32,
    p2_score: u32,
    nsimilar: u128,
    turn: bool
}

impl Universe {
    fn new(p1_pos: u32, p2_pos: u32) -> Self {
        Self {
            p1_pos: p1_pos,
            p2_pos: p2_pos,
            p1_score: 0,
            p2_score: 0,
            nsimilar: 1,
            turn: true
        }
    }

    fn check_winner(&self) -> Option<bool> {
        if self.p1_score >= 21 {
            return Some(true);
        }

        if self.p2_score >= 21 {
            return Some(false);
        }

        None
    }

    fn move_player(&mut self, rollvalue: u32) {
        if self.turn {
            self.p1_pos = movepawn(rollvalue, self.p1_pos);
            self.p1_score += self.p1_pos;
        } else {
            self.p2_pos = movepawn(rollvalue, self.p2_pos);
            self.p2_score += self.p2_pos;
        }

        self.turn = !self.turn;
    }
}

#[derive(Debug)]
struct Multiverse {
    p1_wins: u128,
    p2_wins: u128
}

impl Multiverse {
    fn winner_score(&self) -> u128 {
        if self.p1_wins > self.p2_wins {self.p1_wins} else {self.p2_wins}
    }
}

fn multiverse(universe: Universe, m: &mut Multiverse) {
    for (i, freq) in FREQ.iter().enumerate() {
        let rollvalue: u32 = i as u32 + 3;
        let mut u = universe.clone();
        u.move_player(rollvalue);

        if let Some(p1_won) = u.check_winner() {
            if p1_won {
                m.p1_wins += *freq * u.nsimilar;
            } else {
                m.p2_wins += *freq *  u.nsimilar;
            }
        } else {
            u.nsimilar *= freq;
            multiverse(u, m);
        }
    }
}

pub fn p21_2(data: &Vec<String>) -> u128 {
    let (p1_pos, p2_pos) = positions(data);
    let mut m = Multiverse { p1_wins: 0, p2_wins: 0 };
    let u = Universe::new(p1_pos, p2_pos);

    multiverse(u, &mut m);
    m.winner_score()
}
