#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Bot {
    x: i32,
    y: i32,
    z: i32,
    r: i32,
}

fn parse_input(input: &str) -> Bot {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"pos=<([-]?\d+),([-]?\d+),([-]?\d+)>, r=(\d+)"
        )
        .unwrap();
    }

    let caps = RE.captures(input).unwrap();
    let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let z = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let r = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
    Bot { x, y, z, r }
}

fn q1(bots: &mut Vec<Bot>) {
    let largest_r = bots.iter().map(|b| b.r).max().unwrap();
    let largest_r_bot_idx = bots.iter().position(|b| b.r == largest_r).unwrap();
    let mut q1_counter = 0;

    for (idx, ref b) in bots.iter().enumerate() {
        let md = (b.x - bots[largest_r_bot_idx].x).abs()
            + (b.y - bots[largest_r_bot_idx].y).abs()
            + (b.z - bots[largest_r_bot_idx].z).abs();
        if md <= largest_r {
            q1_counter += 1;
        }
    }
    println!("result of q01 is {}", q1_counter);
}

fn main() {
    let path = format!("./input/{}", "day23.txt");

    let mut bots: Vec<Bot> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|s| parse_input(&s))
        .collect();

    q1(&mut bots);
}
