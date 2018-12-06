use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

fn react(input: &str) -> usize {
    let mut stack: VecDeque<char> = VecDeque::with_capacity(input.len());
    input.chars().for_each(|c| match stack.back() {
        Some(&last_c) => {
            if (last_c.clone().to_ascii_lowercase() == c.clone().to_ascii_lowercase())
                && (last_c.is_ascii_uppercase() != c.is_ascii_uppercase())
            {
                stack.pop_back();
            } else {
                stack.push_back(c);
            }
        }
        None => {
            stack.push_back(c);
        }
    });
    stack.len()
}

fn main() {
    let input = {
        let mut input = String::new();
        let path = format!("./input/{}", "day05.txt");
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut input).unwrap();
        input
    };

    // q1
    println!("result of q01 is {}", react(&input));

    // q2
    // ranges of chars are tricky because not all 32-bit values represent valid Unicode characters.
    // Because of this, char does not implement the Add trait which is required by the Iterator implementation for RangeInclusive.
    let min_count = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|c| input.replace(c, "").replace(c.to_ascii_uppercase(), ""))
        .map(|s| react(&s))
        .min()
        .unwrap();
    println!("result of q02 is {}", min_count);
}
