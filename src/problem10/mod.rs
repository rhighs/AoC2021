#[derive(Debug, Copy, Clone)]
struct Chunk {
    token: char,
    starting: bool
}

impl Chunk {
    pub fn new(token: char) -> Self {
        let starting = token == '('
            || token == '['
            || token == '{'
            || token == '<';

        Self { token, starting }
    }

    pub fn close(&self, other: &Chunk) -> u8 {
        let other_family = match other.token {
            ')' => Some('('),
            ']' => Some('['),
            '}' => Some('{'),
            '>' => Some('<'),
            _ => None
        };

        if other_family.is_none() {
            return 3;
        }

        if self.token == other_family.unwrap() {1} else {2}
    }

    pub fn token_value(&self) -> Option<u32> {
        match self.token {
            ')' => Some(3),
            ']' => Some(57),
            '}' => Some(1197),
            '>' => Some(25137),
            _ =>  None
        }
    }
}

fn paren(line: &Vec<Chunk>, pos: usize) -> (bool, usize) {
    if pos >= line.len() {
        return (false, 0);
    }

    let current = &line[pos];

    if current.starting {
        let payload = paren(&line, pos + 1);
        if !payload.0 {
            return payload;
        }

        let result = payload.1;

        if current.close(&line[result]) == 1 {
            return paren(&line, result + 1);
        } else {
            return (false, result);
        }
    } else {
        return (true, pos);
    }
}  

fn paren2(line: &Vec<Chunk>, open_brackets: &mut Vec<Chunk>, pos: usize) -> (bool, usize) {
    if pos >= line.len() {
        return (false, 0);
    }

    let current = &line[pos];

    if current.starting {
        open_brackets.push(current.clone());
        let i = open_brackets.len() - 1;
        let payload = paren2(&line, open_brackets, pos + 1);

        if !payload.0 {
            return payload;
        }

        let result = payload.1;
        return if current.close(&line[result]) == 1 {
            open_brackets.remove(i);
            paren2(&line, open_brackets, result + 1)
        } else {
            open_brackets.remove(i);
            (false, result)
        }
    } else {
        return (true, pos);
    }
}  

fn make_chunks(data: &Vec<String>) -> Vec<Vec<Chunk>> {
    let mut chunks = Vec::new();
    for line in data {
        let mut chunk = Vec::new();
        for token in line.chars() {
            chunk.push(Chunk::new(token));
        }
        chunks.push(chunk);
    }
    chunks
}

pub fn p10_1(data: &Vec<String>) -> u32 {
    let chunks = make_chunks(data);
    let mut sum = 0;
    for line in &chunks {
        let result = paren(line, 0);
        if !result.0 && result.1 != 0 {
            let value = line[result.1].token_value();
            if value.is_some() {
                sum += value.unwrap();
            }
        }
    }
    sum
}

pub fn p10_2(data: &Vec<String>) -> u64 {
    let chunks = make_chunks(data);

    let incomplete: Vec<Vec<Chunk>> = chunks
        .into_iter()
        .filter(|line| {
            let result = paren(line, 0);
            !result.0 && result.1 == 0
        }).collect();

    let mut scores = Vec::new();
    for line in &incomplete {
        let mut open_brackets = Vec::new();
        paren2(line, &mut open_brackets, 0);
        let mut score: u64 = 0;
        for paren in open_brackets.iter().rev() {
            score *= 5;
            score += match paren.token {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0
            };
        }
        scores.push(score);
    }

    scores.sort();
    scores[scores.len() / 2]
}

