const INF: u32 = 1000;

fn into_grid(data: &Vec<String>) -> Vec<Vec<u32>> {
    let mut grid = Vec::new();
    for string in data {
        let row = string.chars()
            .map(|c| c.to_string().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        grid.push(row);
    }
    grid
}

#[derive(Copy, Clone, Debug)]
struct Edge {
    label: usize,
    current: (usize, usize),
    pred: (usize, usize),
    label_pred: usize,
    cost: u32
}

impl Edge {
    fn new(current: (usize, usize), pred: (usize, usize), cost: u32, grid: &Vec<Vec<u32>>) -> Self {
        Self {
            label: (current.0 * grid.len()) + current.1,
            current,
            pred,
            label_pred: (pred.0 * grid.len()) + pred.1,
            cost
        }
    }
}

pub fn p15_1(data: &Vec<String>) -> u32 {
    let grid = into_grid(data);
    let mut tree = Vec::new();
    let new_grid = vec![vec![0u32; grid[0].len()]; grid.len()];
    let dists_len = grid.len() * grid[0].len();
    let mut dist: Vec<u32> = vec![INF; dists_len];
    dist[0] = 0;

    let mut closed_list = Vec::new();
    closed_list.push(0);

    tree.push(Edge::new((0, 0), (0, 0), grid[0][0], &grid));

    while closed_list.len() < dists_len {
        let mut adjs = Vec::new();

        for node in closed_list.iter().rev() {
            let i: usize = node / 10;
            let j: usize = node % 10;
            let mut right = None;
            let mut down = None;

            if i + 1 < grid.len() {
                let d = Edge::new((i + 1, j), (i, j), grid[i + 1][j], &grid);
                adjs.push(d);
                down = Some(d);
            }
            if j + 1 < grid[0].len() {
                let r = Edge::new((i, j + 1), (i, j), grid[i][j + 1], &grid);
                adjs.push(r);
                right = Some(r);
            }
        }

        let mut adjs = adjs
            .iter()
            .filter(|a| !closed_list.contains(&a.label))
            .cloned()
            .collect::<Vec<Edge>>();
        adjs.sort_by(|a, b| a.cost.cmp(&b.cost));

        if adjs.len() > 0 {
            let mut min = adjs[0];
            let mut others = adjs
                .iter()
                .filter(|val| val.cost == min.cost)
                .cloned()
                .collect::<Vec<Edge>>();
            let other_min = adjs[1];
            min = others[others.len() - 1];
            println!("Chosen min {:?}, at {:?}", min.cost, min.current);
            closed_list.push(min.label);
            tree.push(min);
        }
    }

    tree.sort_by(|a, b| a.label.cmp(&b.label));

    let mut path = Vec::new();
    let mut current_edge = tree.last().unwrap();
    path.push(current_edge.current);

    loop {
        current_edge = &tree[tree.iter().position(|edge| edge.label == current_edge.label_pred).unwrap()];
        path.push(current_edge.current);
        if current_edge.label == 0 {
            break;
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if path.iter().any(|pos| pos.0 == i && pos.1 == j) {
                print!("*");
            } else {
                print!("{}", grid[i][j]);
            }
        }
        println!("");
    }

    path.iter().map(|a| grid[a.0][a.1]).sum()
}

pub fn p15_2(data: &Vec<String>) -> u32 {
    0
}