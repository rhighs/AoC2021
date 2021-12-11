fn make_grid(data: &Vec<String>) -> Vec<Vec<u32>> {
    let mut grid = Vec::new();
    for string in data {
        let mut line = Vec::new();
        for character in string.chars() {
            line.push(character.to_string().parse::<u32>().unwrap());
        }
        grid.push(line);
    }
    grid
}

fn flash(grid: &mut Vec<Vec<u32>>, i: i32, j: i32, flash_count: &mut u32) {
    if grid[i as usize][j as usize] < 10 {
        return;
    }

    *flash_count += 1;
    grid[i as usize][j as usize] = 0;

    let neighbours = [(i+1, j), (i+1, j-1), (i+1, j+1), (i-1, j), (i-1, j-1), (i-1, j+1), (i, j+1), (i, j-1)];
    for pos in neighbours {
        let pos_valid = pos.0 >= 0 && pos.0 < grid.len() as i32
           && pos.1 >= 0 && pos.1 < grid[0].len() as i32;
        if pos_valid && grid[pos.0 as usize][pos.1 as usize] != 0 {
            grid[pos.0 as usize][pos.1 as usize] += 1;
            flash(grid, pos.0, pos.1, flash_count);
        }
    }
}

fn step(grid: &mut Vec<Vec<u32>>) -> u32 {
    let row_len = grid[0].len();
    let col_len = grid.len();
    let mut flash_count = 0;

    for i in 0..row_len {
        for j in 0..col_len {
            grid[i][j] += 1;
        }
    }

    for i in 0..row_len {
        for j in 0..col_len {
            flash(grid, i as i32, j as i32, &mut flash_count);
        }
    }

    flash_count
}

pub fn p11_1(data: &Vec<String>) -> u32 {
    let mut grid = make_grid(data);
    let mut total = 0;

    for _ in 0..200 {
        total += step(&mut grid);
    }

    total
}

pub fn p11_2(data: &Vec<String>) -> u32 {
    let mut grid = make_grid(data);
    let mut n = 0;
    while step(&mut grid) != 10 * 10 {
        n += 1;
    }
    n + 1
}

