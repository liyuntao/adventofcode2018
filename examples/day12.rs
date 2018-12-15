use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

fn parse_all_rules(inputs: Vec<String>) -> HashMap<String, bool> {
    let mut rules: HashMap<String, bool> = HashMap::new();
    inputs.iter().for_each(|line| {
        let i = line[0..5].to_string();
        rules.insert(i, line.chars().nth(9).unwrap() == '#');
    });
    rules
}

fn trigger_gen(state: &Vec<char>, rules: &HashMap<String, bool>) -> Vec<char> {
    let mut next_state = vec!['.'; state.len()];

    for i in 0..state.len() - 5 {
        let give_me_five: String = state[i..i + 5].into_iter().collect();
        if let Some(&exist) = rules.get(&give_me_five) {
            if exist {
                next_state[i + 2] = '#';
            } else {
                next_state[i + 2] = '.';
            }
        }
    }
    next_state
}

fn q1(init_state: &str, gen_times: usize, rules: &HashMap<String, bool>) -> i32 {
    let init_offset = 5i32;

    let mut begin_str = ".".repeat(init_offset as usize);
    begin_str.push_str(init_state);
    begin_str.push_str(&".".repeat(gen_times + 10));

    let mut last_vec: Vec<char> = begin_str.chars().collect();

    (1..=gen_times).for_each(|i| {
        last_vec = trigger_gen(&last_vec, rules);
//        println!("DEBUG {} \t {}", i, String::from_iter(last_vec.clone()));
    });

    // count sum
    last_vec
        .iter()
        .enumerate()
        .map(|(i, &c)| if c == '#' { i as i32 - init_offset } else { 0 })
        .sum()
}

fn main() {
    let path = format!("./input/{}", "day12.txt");

    let vec: Vec<String> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let rules: HashMap<String, bool> = parse_all_rules(vec);

    let init_state = ".#..##..#.....######.....#....####.##.#.#...#...##.#...###..####.##.##.####..######......#..##.##.##";
    let q1_res = q1(init_state, 20, &rules);
    println!("result of q01 is {}", q1_res);
}
