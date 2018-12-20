#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};

const GRID_WIDTH: usize = 600;
const GRID_HEIGHT: usize = 2000;

fn parse_line(line: &str) -> (char, usize, usize, usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\w)=(\d+), \w=(\d+)..(\d+)").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    let c = caps[1].parse::<char>().unwrap();
    let u1 = caps[2].parse::<usize>().unwrap();
    let u2 = caps[3].parse::<usize>().unwrap();
    let u3 = caps[4].parse::<usize>().unwrap();
    (c, u1, u2, u3)
}

fn parse_all(path: String) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; GRID_WIDTH]; GRID_HEIGHT];
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|line| parse_line(&line))
        .for_each(|tp| {
            for i in tp.2..=tp.3 {
                if tp.0 == 'x' {
                    grid[i][tp.1] = '#';
                } else {
                    grid[tp.1][i] = '#';
                }
            }
        });
    grid
}

fn seek_boundary(grid: &Vec<Vec<char>>) -> (usize, usize, usize, usize) {
    let mut min_x: usize = GRID_WIDTH;
    let mut max_x: usize = 0;
    let mut min_y: usize = GRID_HEIGHT;
    let mut max_y: usize = 0;
    for j in 0..GRID_HEIGHT {
        for i in 0..GRID_WIDTH {
            if grid[j][i] == '#' {
                min_x = min(min_x, i);
                max_x = max(max_x, i);
                min_y = min(min_y, j);
                max_y = max(max_y, j);
            }
        }
    }
    (min_x - 1, max_x + 1, min_y, max_y) // fuck this!
}

fn draw(grid: &Vec<Vec<char>>) {
    let (min_x, max_x, min_y, max_y) = seek_boundary(&grid);
    for j in min_y..=max_y {
        print!("{:04}  ", j);
        for i in min_x..=max_x {
            print!("{}", grid[j][i]);
        }
        println!();
    }
}

fn count_tiles(grid: &Vec<Vec<char>>) -> (usize, usize) {
    let mut total = 0;
    let mut spring = 0;
    let (min_x, max_x, min_y, max_y) = seek_boundary(&grid);
    for j in min_y..=max_y {
        for i in min_x..=max_x {
            if grid[j][i] == '|' {
                total += 1;
            } else if grid[j][i] == '~' {
                total += 1;
                spring += 1;
            }
        }
    }
    (total, spring)
}

fn is_exceed(x: usize, y: usize, bd: (usize, usize, usize, usize)) -> bool {
    !(x >= bd.0 && x <= bd.1 && y >= bd.2 && y <= bd.3)
}

fn flow(
    x_start: usize,
    y_start: usize,
    grid: &mut Vec<Vec<char>>,
    bd: (usize, usize, usize, usize),
) {
    if is_exceed(x_start, y_start, bd) || grid[y_start + 1][x_start] == '~' {
        return;
    }

    for j in y_start..=bd.3 {
        // vertical flow
        if grid[j][x_start] == '.' || grid[j][x_start] == '|' {
            grid[j][x_start] = '|';
        } else if grid[j][x_start] == '#' || grid[j][x_start] == '~' {
            // hit bottom wall, start horizon flow
            {
                let mut hit_left_wall = false;
                let mut hit_right_wall = false;

                for i in (bd.0..x_start).rev() {
                    if grid[j][i] == '#' || grid[j][i] == '~' {
                        if grid[j - 1][i] != '#' && grid[j - 1][i] != '~' {
                            grid[j - 1][i] = '|';
                        } else {
                            hit_left_wall = true;
                            break;
                        }
                    } else {
                        flow(i, j - 1, grid, bd);
                        if grid[j][i] != '~' {
                            break;
                        }
                    }
                }

                for i in x_start..=bd.1 {
                    if grid[j][i] == '#' || grid[j][i] == '~' {
                        if grid[j - 1][i] != '#' && grid[j - 1][i] != '~' {
                            grid[j - 1][i] = '|';
                        } else {
                            hit_right_wall = true;
                            break;
                        }
                    } else {
                        flow(i, j - 1, grid, bd);
                        if grid[j][i] != '~' {
                            break;
                        }
                    }
                }

                if hit_left_wall && hit_right_wall {
                    for i in (bd.0..x_start).rev() {
                        if grid[j - 1][i] == '|' {
                            grid[j - 1][i] = '~';
                        } else {
                            break;
                        }
                    }
                    for i in x_start..=bd.1 {
                        if grid[j - 1][i] == '|' {
                            grid[j - 1][i] = '~';
                        } else {
                            break;
                        }
                    }
                    flow(x_start, y_start, grid, bd);
                }
            }
            break;
        }
    }
}

fn main() {
    let path = format!("./input/{}", "day17.txt");
    let mut grid = parse_all(path);
    let bd = seek_boundary(&grid);
    flow(500, bd.2, &mut grid, bd);
    draw(&grid);
    let res = count_tiles(&grid);
    println!("result of q01 is {}", res.0);
    println!("result of q02 is {}", res.1);
}
