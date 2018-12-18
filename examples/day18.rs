use std::fs::File;
use std::io::{BufRead, BufReader};

struct UnitRound {
    tree_count: usize,
    lumberyard_count: usize,
    open_count: usize,
}

fn parse_all(path: String) -> Vec<Vec<char>> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn tick(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = vec![vec!['.'; grid[0].len()]; grid.len()];
    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            res[j][i] = transform(grid, i, j);
        }
    }
    res
}

fn transform(grid: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    let status = unit_round(grid, x, y);

    if grid[y][x] == '.' && status.tree_count >= 3 {
        '|'
    } else if grid[y][x] == '|' && status.lumberyard_count >= 3 {
        '#'
    } else if grid[y][x] == '#' && !(status.lumberyard_count >= 1 && status.tree_count >= 1) {
        '.'
    } else {
        grid[y][x]
    }
}

fn unit_round(grid: &Vec<Vec<char>>, x: usize, y: usize) -> UnitRound {
    let x = x as i32;
    let y = y as i32;
    let max_x: i32 = grid[0].len() as i32;
    let max_y: i32 = grid.len() as i32;

    let mut res = UnitRound {
        tree_count: 0,
        lumberyard_count: 0,
        open_count: 0,
    };

    for t in &[
        (x, y + 1),
        (x, y - 1),
        (x + 1, y),
        (x - 1, y),
        (x + 1, y + 1),
        (x - 1, y - 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
    ] {
        if 0 <= t.0 as i32 && max_x > t.0 as i32 && 0 <= t.1 as i32 && max_y > t.1 as i32 {
            if grid[t.1 as usize][t.0 as usize] == '.' {
                res.open_count += 1;
            } else if grid[t.1 as usize][t.0 as usize] == '#' {
                res.lumberyard_count += 1;
            } else {
                res.tree_count += 1;
            }
        }
    }
    res
}

fn count(grid: &Vec<Vec<char>>) -> usize {
    let mut woods = 0;
    let mut lumberyards = 0;
    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            if grid[j][i] == '|' {
                woods += 1;
            } else if grid[j][i] == '#' {
                lumberyards += 1;
            }
        }
    }
    woods * lumberyards
}

fn draw(grid: &Vec<Vec<char>>) {
    for j in 0..grid.len() {
        print!("{:03}  ", j);
        for i in 0..grid[0].len() {
            print!("{}", grid[j][i]);
        }
        println!();
    }
    println!();
}

fn main() {
    let path = format!("./input/{}", "day18.txt");

    // q1
    let mut grid = parse_all(path);
    for _i in 1..=10 {
        grid = tick(&grid);
    }
    //        draw(&grid)
    println!("result of q01 is {}", count(&grid));

    // q2
    for i in 11..=1028 {
        grid = tick(&grid);
        if i > 970 {
            // value repeats every 28 iterations
            // search it by yourself
            println!("{}  {}", count(&grid), i);
        }
    }
}
