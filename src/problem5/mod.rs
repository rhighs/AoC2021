#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    m: i32
}

#[derive(Debug, Copy, Clone)]
struct Line {
    a: Point,
    b: Point
}

const MAX_SIZE: usize = 1000;

impl Point {
    fn new(x: i32, y: i32, m: i32) -> Self {
        Self { x, y, m }
    }
}

impl Line {
    fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    fn between_hv(&self) -> Option<Vec<Point>> {
        let mut points = Vec::new();

        let make_range = |x1, x2| -> Vec<i32> {
            let sum = x1 + x2;
            let max = std::cmp::max(x1, x2);
            let min = sum - max;
            (min..(max + 1)).collect()
        };

        if self.a.x == self.b.x {
            let range = make_range(self.a.y, self.b.y);
            for i in range {
                points.push(Point::new(self.a.x, i, 0));
            }
        } else if self.a.y == self.b.y {
            let range = make_range(self.a.x, self.b.x);
            for i in range {
                points.push(Point::new(i, self.a.y, 0));
            }
        } else {
            return None;
        }

        Some(points)
    }

    fn between_all(&self) -> Option<Vec<Point>> {
        if let Some(points) = self.between_hv() {
            return Some(points);
        }

        let mut points: Vec<Point> = Vec::new();
        let offset = (self.a.x - self.b.x).abs();

        match (self.a.x < self.b.x, self.a.y < self.b.y) {
            (true, true) => for i in 0..(offset + 1) { points.push(Point::new(i, i, 0)) },
            (false, true) => for i in 0..(offset + 1) { points.push(Point::new(-i, i, 0)) },
            (true, false) => for i in 0..(offset + 1) { points.push(Point::new(i, -i, 0)) },
            (false, false) => for i in 0..(offset + 1) { points.push(Point::new(-i, -i, 0)) },
        }

        for point in &mut points {
            point.x += self.a.x;
            point.y += self.a.y;
        }

        Some(points)
    }
}

fn parse_lines(data: &Vec<String>) -> Vec<Line> {
    let mut lines = Vec::new();

    for string in data {
        let points: Vec<&str> = string.split(" -> ").collect();
        let coords_a: Vec<&str> = points[0].split(",").collect();
        let coords_b: Vec<&str> = points[1].split(",").collect();
        let point_a = Point::new(
            coords_a[0].parse().unwrap(),
            coords_a[1].parse().unwrap(),
            0
            );
        let point_b = Point::new(
            coords_b[0].parse().unwrap(),
            coords_b[1].parse().unwrap(),
            0
            );
        lines.push(Line::new(point_a, point_b));
    }

    lines
}

fn parse_points(data: &Vec<String>) -> Vec<Point> {
    let mut points_parsed: Vec<Point> = Vec::new();

    for string in data {
        let points_str: Vec<&str> = string.split(" -> ").collect();
        let coords_a: Vec<&str> = points_str[0].split(",").collect();
        let coords_b: Vec<&str> = points_str[1].split(",").collect();
        let point_a = Point::new(
            coords_a[0].parse().unwrap(),
            coords_a[1].parse().unwrap(),
            0
            );
        let point_b = Point::new(
            coords_b[0].parse().unwrap(),
            coords_b[1].parse().unwrap(),
            0
            );
        points_parsed.push(point_a);
        points_parsed.push(point_b);
    }

    points_parsed
}

fn filter_diagonal(lines: &Vec<Line>) -> Vec<Line> {
    lines
        .iter()
        .filter(|line| line.a.x == line.b.x || line.a.y == line.b.y)
        .map(|line| *line)
        .collect()
}
 
pub fn p5_1(data: &Vec<String>) -> i32 {
    let lines = parse_lines(data);
    let filtered = filter_diagonal(&lines);
    let mut grid = vec![vec![0; MAX_SIZE]; MAX_SIZE];

    for line in filtered {
        let between = line.between_hv();
        if between.is_some() {
            for point in between.unwrap() {
                grid[point.x as usize][point.y as usize] += 1;
            }
        }
    }

    let mut sum: i32 = 0;
    for i in 0..MAX_SIZE {
        for j in 0..MAX_SIZE {
            if grid[i][j] > 1 {
                sum += 1;
            }
        }
    }

    sum
}

pub fn p5_2(data: &Vec<String>) -> i32 {
    let lines = parse_lines(data);
    let mut grid = vec![vec![0; MAX_SIZE]; MAX_SIZE];

    for line in lines {
        let between = line.between_all();
        if between.is_some() {
            for point in between.unwrap() {
                grid[point.x as usize][point.y as usize] += 1;
            }
        }
    }

    let mut sum: i32 = 0;
    for i in 0..MAX_SIZE {
        for j in 0..MAX_SIZE {
            if grid[j][i] > 1 {
                sum += 1;
            }
        }
    }

    sum
}

