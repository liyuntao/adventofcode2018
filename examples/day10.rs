#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Node {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Node {
    fn tick(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
}

fn parse(input: &str) -> Node {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"position=<\s*([-]?\d+),\s*([-]?\d+)> velocity=<\s*([-]?\d+),\s*([-]?\d+)>"
        )
        .unwrap();
    }

    let caps = RE.captures(input).unwrap();
    let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let vx = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let vy = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
    Node { x, y, vx, vy }
}

fn main() {
    let path = format!("./input/{}", "day10.txt");

    let vec: Vec<String> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut nodes: Vec<Node> = vec.iter().map(|x| parse(x)).collect();

    for counter in 1.. {
        nodes.iter_mut().for_each(|node| node.tick());

        if nodes[..nodes.len() - 1]
            .iter()
            .zip(nodes[1..].iter())
            .all(|pair| (pair.0.y - pair.1.y).abs() <= 10)
        {
            println!("result of q02 is {}", counter);
            break;
        }
    }

    let min_x = nodes.iter().map(|n| n.x).min().unwrap();
    let min_y = nodes.iter().map(|n| n.y).min().unwrap();

    let mut visualized: [[char; 100]; 10] = [['.'; 100]; 10];
    nodes
        .iter()
        .for_each(|n| visualized[(n.y - min_y) as usize][(n.x - min_x) as usize] = '#');
    visualized.iter().for_each(|line| {
        line.iter().for_each(|c| print!("{}", c));
        println!();
    });
}
