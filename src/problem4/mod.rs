type Board = Vec<Vec<u32>>;

fn extract_numbers(string: &String, sep: &'static str) -> Vec<u32> {
    let mut cleaned = string.replace("  ", " ");
    cleaned = cleaned.trim_start().to_owned();
    cleaned = cleaned.trim_end().to_owned();

    cleaned
        .split(sep)
        .collect::<Vec<&str>>()
        .iter()
        .map(|&b| b.to_string().parse::<u32>().unwrap() + 1)
        .collect()
}

fn parse_boards(data: &Vec<String>) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    let mut current_board: Board = Vec::new();

    for i in 1..(data.len()) {
        if !data[i].is_empty() {
            current_board.push(extract_numbers(&data[i], " "));
        } else if current_board.len() != 0 {
            boards.push(current_board.clone());
            current_board = Vec::new();
        }
    }

    if current_board.len() != 0 {
        boards.push(current_board.clone());
    }

    boards
}

fn check_winner(boards: &Vec<Board>) -> (bool, Vec<usize>) {
    let mut winners = Vec::new();
    for (i, board) in boards.iter().enumerate() {
        for line in board {
            if line.iter().all(|&value| value == 0) {
                winners.push(i)
            }
        }

        for col in 0..5 {
            let mut col_streak = true;
            for line in board {
                if col_streak{
                    col_streak = line[col] == 0;
                }
            }
            if col_streak {
                winners.push(i)
            }
        }
    }

    winners.sort();
    winners.dedup();
    (winners.len() > 0, winners)
}

fn mark_boards(number: u32, boards: &mut Vec<Board>) {
    for board in boards {
        for line in board {
            for i in 0..line.len() {
                if line[i] == number {
                    line[i] = 0;
                }
            }
        }
    }
}

fn board_sum(board: &Board) -> u32 {
    let mut sum = 0;
    for line in board {
        for i in line {
            let mut x = *i;
            if x != 0{
                x -= 1;
            }
            sum += x;
        }
    }
    sum
}

pub fn p4_1(data: &Vec<String>) -> u32 {
    let numbers = extract_numbers(&data[0], ",");
    let mut boards = parse_boards(data);

    let mut board_number = 0;
    let mut last_number = 0;
    for number in numbers {
        mark_boards(number, &mut boards);
        match check_winner(&boards) {
            (true, bno) => { board_number = bno[bno.len() - 1]; last_number = number; break; },
            _ => ()
        }
    }

    (last_number - 1) * board_sum(&boards[board_number])
}

pub fn p4_2(data: &Vec<String>) -> u32 {
    let mut last_board_sum = 0;
    let mut last_number = 0;
    let mut boards = parse_boards(data);
    let numbers = extract_numbers(&data[0], ",");

    for number in numbers {
        mark_boards(number, &mut boards);
        match check_winner(&boards) {
            (true, mut bno) => { 
                last_number = number; 
                last_board_sum = board_sum(&boards[*bno.last().unwrap()]);
                bno.reverse();
                for b in bno {
                    boards.remove(b); 
                }
            },
            _ => ()
        }
    }

    (last_number - 1) * last_board_sum
}
