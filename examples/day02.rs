use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn parse_count(s: &str) -> (i32, i32) {
    // use the vec! macro to initialize a vector of any constant value of a given length
    let mut counter: Vec<i32> = vec![0; 26];
    s.chars()
        .for_each(|c| counter[(c as usize) - ('a' as usize)] += 1);

    // Don't iterate over references, iterate over the values themselves.
    //    let owned_iter = counter.iter();
    //    let two = owned_iter.filter(|&&i| i == 2).count();

    let has_two = if counter.contains(&2) { 1 } else { 0 };
    let has_three = if counter.contains(&3) { 1 } else { 0 };

    return (has_two, has_three);
}

fn valid_str(a: &str, b: &str) -> Option<usize> {
    let mut pos = 0;
    let mut counter = 0;
    let size = a.len();
    for i in 0..(size - 1) {
        if counter > 1 {
            return None;
        }

        let c1 = a.chars().nth(i).unwrap();
        let c2 = b.chars().nth(i).unwrap();

        if c1 != c2 {
            pos = i;
            counter += 1;
        }
    }

    println!("debug {}  {} pos={}", a, b, pos);

    Some(pos)
}

fn main() {
    let path = format!("./input/{}", "day02.txt");

    let vec1: Vec<String> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    // q1
    let vec: Vec<(i32, i32)> = vec1.iter().map(|ref l| parse_count(&l)).collect();
    let res: (i32, i32) = vec
        .iter()
        .fold((0, 0), |acc, &t| (acc.0 + t.0, acc.1 + t.1));
    println!("result of q01 is {}", res.0 * res.1);

    // q2
    let size = vec1.len();
    for i in 0..(size - 1) {
        for j in (i + 1)..size {
            if let Some(pos) = valid_str(&vec1[i], &vec1[j]) {
                let s = &vec1[i];
                println!("result of q02 is {}{}", &s[..pos], &s[(pos + 1)..]);
                return;
            }
        }
    }
}
