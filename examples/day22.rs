use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Unit {
    geo_idx: usize,
    ero_lv: usize,
    r_type: usize,
}

fn gen_grid(tar_x: usize, tar_y: usize, depth: usize) -> Vec<Vec<Unit>> {
    let mut grid = vec![
        vec![
            Unit {
                geo_idx: 0,
                ero_lv: 0,
                r_type: 0
            };
            tar_x + 1
        ];
        tar_y + 1
    ];
    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            let geo_idx = if (i == 0 && j == 0) || (i == tar_x && j == tar_y) {
                0
            } else if j == 0 {
                i * 16807
            } else if i == 0 {
                j * 48271
            } else {
                grid[j - 1][i].ero_lv * grid[j][i - 1].ero_lv
            };
            let ero_lv = (geo_idx + depth) % 20183;
            let r_type = ero_lv % 3;
            grid[j][i] = Unit {
                geo_idx,
                ero_lv,
                r_type,
            };
        }
    }
    grid
}

fn draw(grid: &Vec<Vec<Unit>>) {
    for j in 0..grid.len() {
        print!("{:03}  ", j);
        for i in 0..grid[0].len() {
            let c = match grid[j][i].r_type {
                0 => '.',
                1 => '=',
                _ => '|',
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn count(grid: &Vec<Vec<Unit>>) -> usize {
    let mut counter = 0;
    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            counter += grid[j][i].r_type;
        }
    }
    counter
}

fn main() {
    // test
    assert_eq!(114, count(&gen_grid(10, 10, 510)));

    // q1
    let grid = gen_grid(14, 760, 7863);
    draw(&grid);
    println!("{}", count(&grid));
}
