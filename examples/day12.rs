use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

const INIT_OFFSET: i64 = 5;

fn parse_all_rules(inputs: Vec<String>) -> HashMap<String, bool> {
    let mut rules: HashMap<String, bool> = HashMap::new();
    inputs.iter().for_each(|line| {
        let i = line[0..5].replace(".", " ").to_string();
        rules.insert(i, line.chars().nth(9).unwrap() == '#');
    });
    rules
}

fn trigger_gen(state: &Vec<char>, rules: &HashMap<String, bool>) -> Vec<char> {
    let mut next_state = vec![' '; state.len()];

    for i in 0..state.len() - 5 {
        let give_me_five: String = state[i..i + 5].into_iter().collect();
        if let Some(&exist) = rules.get(&give_me_five) {
            if exist {
                next_state[i + 2] = '#';
            } else {
                next_state[i + 2] = ' ';
            }
        }
    }
    next_state
}

fn count_sum(input: &Vec<char>) -> i64 {
    input
        .iter()
        .enumerate()
        .map(|(i, &c)| if c == '#' { i as i64 - INIT_OFFSET } else { 0 })
        .sum()
}

fn q1(init_state: &str, rules: &HashMap<String, bool>) -> i64 {
    let gen_times = 20;

    let mut begin_str = " ".repeat(INIT_OFFSET as usize);
    begin_str.push_str(init_state);
    begin_str.push_str(&" ".repeat(gen_times + 10));

    let mut last_vec: Vec<char> = begin_str.chars().collect();

    (1..=gen_times).for_each(|i| {
        last_vec = trigger_gen(&last_vec, rules);
        println!("DEBUG {} \t {}", i, String::from_iter(last_vec.clone()));
    });

    count_sum(&last_vec)
}

fn q2(init_state: &str, rules: &HashMap<String, bool>) -> i64 {
    let mut begin_str = " ".repeat(INIT_OFFSET as usize);
    begin_str.push_str(init_state);
    begin_str.push_str(&" ".repeat(200 + 10));

    let mut last_vec: Vec<char> = begin_str.chars().collect();

    for i in 1.. {
        let cur_gen_visualized = String::from_iter(last_vec.clone());
        last_vec = trigger_gen(&last_vec, rules);
        let next_gen_visualized = String::from_iter(last_vec.clone());

        if cur_gen_visualized.trim() == next_gen_visualized.trim() {
            let cur_sum = count_sum(&last_vec);
            let next_sum = count_sum(&trigger_gen(&last_vec, rules));
            let sum_diff = next_sum - cur_sum;
            return cur_sum + (50000000000 - i) * sum_diff;
        }
    }
    -1 // unreachable for valid aoc input
}

fn solution(input: Vec<String>, init_state: &str) {
    let rules: HashMap<String, bool> = parse_all_rules(input);

    let init_state_handled = init_state.replace('.', " ");

    let q1_res = q1(&init_state_handled, &rules);
    println!("result of q01 is {}", q1_res);

    let q2_res = q2(&init_state_handled, &rules);
    println!("result of q02 is {}", q2_res);
}

fn main() {
    let path = format!("./input/{}", "day12.txt");

    let vec: Vec<String> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    solution(vec,
             ".#..##..#.....######.....#....####.##.#.#...#...##.#...###..####.##.##.####..######......#..##.##.##");
}
