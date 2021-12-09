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
        row.insert(0, 10);
        row.push(10);
    }
    let mut padding = Vec::new();
    for _ in 0..grid[0].len() {
        padding.push(10);
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
    0
}
