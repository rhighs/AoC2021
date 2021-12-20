use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::cmp::Ord;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Node {
    me: (i64, i64),
    cost: u32, 
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Node {
    fn lt(&self, other: &Node) -> bool {
        self.cost < other.cost
    }

    fn le(&self, other: &Node) -> bool {
        self.cost <= other.cost
    }

    fn gt(&self, other: &Node) -> bool {
        self.cost > other.cost
    }

    fn ge(&self, other: &Node) -> bool {
        self.cost >= other.cost
    }

    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Node {
    fn new(me: (i64, i64), cost: u32) -> Self {
        Self { me, cost }
    }
}

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

fn solve(grid: &Vec<Vec<u32>>) -> u32 {
    let start = Node::new((0,0), 0);
    let mut distances = BinaryHeap::new();
    let mut map = HashMap::new();
    distances.push(Reverse(start));
    map.insert(start.me, start.cost);

    let visit = |node: &Node, di: i64, dj: i64, map: &HashMap<(i64, i64), u32>| -> Option<Node> {
        let (i, j) = node.me;
        let me = (i + di, j + dj);

        if me.0 >= 0 && me.1 >= 0 && (me.0 as usize) < grid.len() && (me.1 as usize) < grid[0].len() {
            let cost = map.get(&((me.0 as i64), (me.1 as i64)));
            if cost.is_some() {
                let cost = *cost.unwrap();
                if cost > node.cost + grid[me.0 as usize][me.1 as usize] {
                    let node = Node::new(me, node.cost);
                    return Some(node);
                }
            } else {
                let cost = node.cost + grid[me.0 as usize][me.1 as usize];
                let node = Node::new(me, cost);
                return Some(node);
            }
        }

        None
    };

    let offsets = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    while distances.len() > 0 {
        let min = distances.pop().unwrap();

        for o in offsets {
            let add = visit(&min.0, o.0, o.1, &map);
            if add.is_some() {
                let add = add.unwrap();
                distances.push(Reverse(add));
                map.insert(add.me, add.cost);
            }
        }
    }

    let cost = map.get(&(grid.len() as i64 - 1, grid[0].len() as i64 - 1)).unwrap();
    *cost
}

fn mirror_grid(grid: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut new_grid = Vec::new();

    let inc = |row: &Vec<u32>, by: u32| -> Vec<u32> {
        let mut new_row = Vec::new();
        for i in row {
            let i = if *i + by < 10 {
                *i + by
            } else {
                1 + (*i + by) % 10
            };
            new_row.push(i);
        };
        new_row
    };

    for row in grid {
        let mut new_row = Vec::new();
        for increment in 0..5 {
            new_row.append(&mut inc(&row, increment));
        }
        new_grid.push(new_row);
    }

    let mut final_grid = Vec::new();
    for increment in 0..5 {
        let mut append_grid = Vec::new();
        for row in &new_grid {
            append_grid.push(inc(&row, increment));
        }
        final_grid.append(&mut append_grid);
    }

    final_grid
}

pub fn p15_1(data: &Vec<String>) -> u32 {
    let grid = into_grid(data);
    solve(&grid)
}

pub fn p15_2(data: &Vec<String>) -> u32 {
    let grid = mirror_grid(&into_grid(data));
    solve(&grid)
}
