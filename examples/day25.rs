#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
    label: i32,
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

fn manhattan_distance(d1: &Node, d2: &Node) -> i32 {
    (d1.a - d2.a).abs()
        + (d1.b - d2.b).abs()
        + (d1.c - d2.c).abs()
        + (d1.d - d2.d).abs()
}

fn parse_input(input: &str) -> Node {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([-]?\d+),([-]?\d+),([-]?\d+),([-]?\d+)").unwrap();
    }

    let caps = RE.captures(input).unwrap();
    let a = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let b = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let c = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let d = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
    Node { label: -1, a, b, c, d }
}

fn solution(nodes: &mut Vec<Node>) {
    let mut group_label = 0;

    while !nodes.is_empty() {
        group_label += 1;
        nodes[0].label = group_label;

        // BFS
        let mut queue = VecDeque::new();
        queue.push_back(0);

        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();

            for i in 0..nodes.len() {
                if i != cur && nodes[i].label == -1 {
                    if manhattan_distance(&nodes[cur], &nodes[i]) <= 3 {
                        nodes[i].label = group_label;
                        queue.push_back(i);
                    }
                }
            }

        }
        nodes.retain(|n| n.label == -1);
    }

    println!("result of q01 is {}", group_label);
}


fn main() {
    let path = format!("./input/{}", "day25.txt");

    let mut nodes: Vec<Node> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|s| parse_input(&s))
        .collect();

    solution(&mut nodes);
}
