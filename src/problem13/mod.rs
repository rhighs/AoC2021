struct Point {
    x: u32,
    y: u32
}

struct Fold {
    at: char,
    value: u32
}

fn dots(data: &Vec<String>) -> (Vec<Point>, Vec<Fold>){
    let mut points = Vec::new();
    let mut folds = Vec::new();

    let output = data.into_iter().fold(Vec::new(), |mut acc, string| {
        if string.is_empty() || string == "" {
            acc.push(Vec::new());
        } else {
            if acc.len() == 0 {
                acc.push(Vec::new());
            }
            acc.last_mut().unwrap().push(string);
        }
        acc
    });

    for string in &output[0] {
        let coords = string.split(",").collect::<Vec<&str>>();
        points.push(Point {
            x: coords[0].parse::<u32>().unwrap(),
            y: coords[1].parse::<u32>().unwrap()
        });
    }

    for fold in &output[1] {
        let splitted = fold.split("=").collect::<Vec<&str>>();
        let value = splitted[1].parse::<u32>().unwrap();
        let at = splitted[0].chars().last().unwrap();
        folds.push(Fold { at, value });
    }

    (points, folds)
}

fn mirror(fold_point: u32, coord: u32) -> u32 {
    if coord > fold_point {
        let distance = coord - fold_point;
        fold_point - distance
    } else {
        coord
    }
}

pub fn p13_1(data: &Vec<String>) -> u32 {
    let input = dots(data);
    let mut points = input.0;
    let folds = input.1;

    let max_x = points.iter().map(|point| point.x).max().unwrap();
    let max_y = points.iter().map(|point| point.y).max().unwrap();
    let mut paper = vec![vec![0u8; max_x as usize + 1]; max_y as usize + 1];

    let fold = &folds[0];
    let mut total = 0;
    let mut last_x_fold = Fold { at: 'x', value: max_x };
    let mut last_y_fold = Fold { at: 'y', value: max_y };

    points = points
        .into_iter()
        .map(|mut point| {
            if fold.at == 'x' {
                point.x = mirror(fold.value, point.x);
                last_x_fold.value = fold.value;
            } else {
                point.y = mirror(fold.value, point.y);
                last_y_fold.value = fold.value;
            };
            point
        }).collect::<Vec<Point>>();

    for point in points {
        paper[point.y as usize][point.x as usize] = 1;
    }

    for y in 0..last_y_fold.value as usize {
        for x in 0..last_x_fold.value as usize {
            total += paper[y][x] as u32;
        }
    }

    total as u32
}

pub fn p13_2(data: &Vec<String>) -> u32 {
    let input = dots(data);
    let mut points = input.0;
    let folds = input.1;

    let max_x = points.iter().map(|point| point.x).max().unwrap();
    let max_y = points.iter().map(|point| point.y).max().unwrap();

    let mut paper = vec![vec![0u8; max_x as usize + 1]; max_y as usize + 1];
    let mut last_x_fold = Fold { at: 'x', value: max_x };
    let mut last_y_fold = Fold { at: 'y', value: max_y };

    for fold in folds {
        points = points
            .into_iter()
            .map(|mut point| {
                if fold.at == 'x' {
                    point.x = mirror(fold.value, point.x);
                    last_x_fold.value = fold.value;
                } else {
                    point.y = mirror(fold.value, point.y);
                    last_y_fold.value = fold.value;
                };
                point
            }).collect::<Vec<Point>>();
    }

    for point in points {
        paper[point.y as usize][point.x as usize] = 1;
    }

    for y in 0..last_y_fold.value as usize {
        for x in 0..last_x_fold.value as usize {
            print!("{}", if paper[y][x] == 1 {'#'} else {'.'});
        }
        println!("");
    }

    69420
}

