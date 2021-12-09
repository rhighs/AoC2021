fn make_grid(data: &Vec<String>) -> Vec<Vec<u32>> {
    let mut grid = Vec::new();
    for string in data {
        let mut line = Vec::new();
        for character in string.chars() {
            line.push(character.to_string().parse::<u32>().unwrap());
        }
        grid.push(line)
    }
    for row in &mut grid {
        row.insert(0, 9);
        row.push(9);
    }
    let mut padding = Vec::new();
    for _ in 0..grid[0].len() {
        padding.push(9);
    }
    grid.insert(0, padding.clone());
    grid.push(padding);
    grid
}

pub fn p9_1(data: &Vec<String>) -> u32 {
    let mut sum = 0;
    let grid = make_grid(data);
    for i in 1..(grid.len() - 1) {
        for j in 1..(grid[0].len() - 1) {
            let value = grid[i][j];
            if value < grid[i - 1][j]
                && value < grid[i][j - 1]
                && value < grid[i][j + 1]
                && value < grid[i + 1][j] {
                    sum += value + 1;
            }
        }
    }
    sum
}

pub fn p9_2(data: &Vec<String>) -> u32 {
    let mut grid = make_grid(data);
    let mut basins = Vec::new();

    loop {
        let mut q = Vec::new();
        let mut point_found = false;
        for i in 1..(grid.len() - 1) {
            for j in 1..(grid[0].len() - 1) {
                if grid[i][j] != 9 {
                    q.push((i, j));
                    point_found = true;
                    break;
                }
            }
            if point_found {
                break;
            }
        }

        if q.len() == 0 {
            break;
        }

        let mut values = Vec::from([grid[q[0].0][q[0].1]]);
        while q.len() > 0 {
            let pos = q[0];
            let up = grid[pos.0 - 1][pos.1];
            let down = grid[pos.0 + 1][pos.1];
            let left = grid[pos.0][pos.1 - 1];
            let right = grid[pos.0][pos.1 + 1];

            if up != 9 {
                q.push((pos.0 - 1, pos.1));
                grid[pos.0 - 1][pos.1] = 9;
                values.push(up);
            }
            if down != 9 {
                q.push((pos.0 + 1, pos.1));
                grid[pos.0 + 1][pos.1] = 9;
                values.push(down);
            }
            if left != 9 {
                q.push((pos.0, pos.1 - 1));
                grid[pos.0][pos.1 - 1] = 9;
                values.push(left);
            }
            if right != 9 {
                q.push((pos.0, pos.1 + 1));
                grid[pos.0][pos.1 + 1] = 9;
                values.push(right);
            } 

            grid[pos.0][pos.1] = 9;
            q.remove(0);
        }

        basins.push(values);
    }

    basins.sort_by(|a, b| a.len().cmp(&b.len()));
    let last = basins[basins.len() - 1].len();
    let last1 = basins[basins.len() - 2].len();
    let last2 = basins[basins.len() - 3].len();
    (last * last1 * last2) as u32
}
