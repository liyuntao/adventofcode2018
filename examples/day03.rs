#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const M_SIZE: usize = 1000;

struct Matrix {
    inner: [[u16; M_SIZE]; M_SIZE],
}

impl Matrix {
    pub fn new() -> Matrix {
        return Matrix {
            inner: [[0; M_SIZE]; M_SIZE],
        };
    }

    pub fn fill(&mut self, margin_left: u32, margin_top: u32, x: u32, y: u32) {
        //        println!("[debug] fill {} {} {} {}", margin_left, margin_top, x, y);
        for i in 0..x {
            for j in 0..y {
                self.inner[(margin_top + j) as usize][(margin_left + i) as usize] += 1;
            }
        }
    }

    pub fn check_overlap(&mut self, margin_left: u32, margin_top: u32, x: u32, y: u32) -> bool {
        for i in 0..x {
            for j in 0..y {
                if self.inner[(margin_top + j) as usize][(margin_left + i) as usize] > 1 {
                    return true;
                }
            }
        }
        false
    }

    fn dots(&self) -> impl Iterator<Item = (usize, usize, &u16)> {
        self.inner
            .iter()
            .enumerate()
            .flat_map(|(x, v)| v.iter().enumerate().map(move |(y, v)| (x, y, v)))
    }

    pub fn count_overlap(&self) -> u32 {
        let mut count = 0;
        for (_x, _y, &v) in self.dots() {
            if v > 1 {
                count += 1
            }
        }
        count
    }
}

fn parse_input(input: &str) -> (&str, u32, u32, u32, u32) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\D+").unwrap();
    }

    let splited: Vec<&str> = RE.split(input).into_iter().collect();

    //    println!("[debug] splited {:?}", splited);

    (
        splited[1],                  // ID
        splited[2].parse().unwrap(), // margin_left
        splited[3].parse().unwrap(), // margin_top
        splited[4].parse().unwrap(),
        splited[5].parse().unwrap(),
    )
}

fn main() {
    let path = format!("./input/{}", "day03.txt");

    let vec: Vec<String> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    // q1
    let mut matrix = Matrix::new();
    vec.iter()
        .map(|s| parse_input(s))
        .for_each(|t| matrix.fill(t.1, t.2, t.3, t.4));
    println!("result of q01 is {}", matrix.count_overlap());

    // q2
    vec.iter().map(|s| parse_input(s)).for_each(|t| {
        if !matrix.check_overlap(t.1, t.2, t.3, t.4) {
            println!("result of q02 is {}", t.0);
        }
    });
}
