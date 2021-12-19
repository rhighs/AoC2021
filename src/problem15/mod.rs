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

#[derive(Debug, Copy, Clone)]
struct Node {
    me: (usize, usize),
    p: (usize, usize),
    cost: u32, 
}

impl Node {
    fn new(me: (usize, usize), p: (usize, usize), cost: u32) -> Self {
        Self { me, p, cost }
    }
}

pub fn p15_1(data: &Vec<String>) -> u32 {
    let grid = into_grid(data);
    let start = Node::new((0,0), (0,0), 0);
    let mut distances: Vec<Node> = Vec::from([start]);
    let mut tree = Vec::new();

    let visit = |node: &Node, di: usize, dj: usize, distances: &mut Vec<Node>, tree: &Vec<Node>| {
        let (i, j) = node.me;
 
        let me = (i + di, j + dj);
        if tree.iter().any(|n| n.me == me) {
            return;
        }

        if i + di < grid.len() && j + dj < grid[0].len() {
            let possible_position = distances
                .iter()
                .position(|n| n.me.0 == i + di && n.me.1 == j + dj);

            if possible_position.is_some() {
                let position = possible_position.unwrap();
                if distances[position].cost > node.cost + grid[i + di][j + dj] {
                    distances[position].cost = node.cost;
                    distances[position].p = node.me;
                }
            } else {
                let cost = node.cost + grid[i + di][j + dj];
                distances.push(Node::new(me, node.me, cost));
            }
        }
    };


    while tree.len() <= grid.len() * grid[0].len() && distances.len() > 0 {
        distances.sort_by(|a, b| a.cost.cmp(&b.cost));
        let min = distances[0];
        tree.push(min);

        visit(&min, 0, 1, &mut distances, &tree);
        visit(&min, 1, 0, &mut distances, &tree);

        distances.remove(0);
    }

    let last = tree
        .iter()
        .find(|node| node.me == (grid.len() - 1, grid[0].len() - 1))
        .unwrap();

    last.cost
}

pub fn p15_2(data: &Vec<String>) -> u32 {
    0
}