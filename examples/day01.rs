use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn read_input(file_name: &str) -> Result<Vec<i64>, Error> {
    let path = format!("./input/{}", file_name);
    let br = BufReader::new(File::open(path)?);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() {
    let numbers = read_input("day01.txt").unwrap();

    let res_q1: i64 = numbers.iter().sum();
    println!("result of q01 is {}", res_q1);

    let mut occurred_frequency = HashSet::new();
    let mut cur_frequency: i64 = 0;

    for i in numbers.iter().cycle() {
        cur_frequency += i;

        if !occurred_frequency.contains(&cur_frequency) {
            occurred_frequency.insert(cur_frequency);
        } else {
            println!("result of q02 is {}", cur_frequency);
            return;
        }
    }
}
