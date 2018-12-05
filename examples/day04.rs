use std::fs::File;
use std::io::{BufRead, BufReader};

enum StateChange {
    BeginShift(String),
    FallAsleep,
    WakeUp,
}

struct Record {
    month: i32,
    day: i32,
    time: i32,
    state: StateChange,
}

fn parse(s: &str) -> Record {}

fn main() {
    let path = format!("./input/{}", "day04.txt");

    let vec: Vec<String> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
}
