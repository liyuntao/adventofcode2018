use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

fn parse_node(input: &Vec<u32>, start_pos: usize) -> (Node, usize, u32) {
    let mut pos = start_pos;
    let mut sum = 0;

    let child_count = input[pos];
    pos += 1;
    let meta_count = input[pos];
    pos += 1;

    let mut res = Node {
        children: Vec::new(),
        metadata: Vec::new(),
    };

    for i in 0..child_count {
        let (child, next_start_pos, child_sum) = parse_node(input, pos);
        sum += child_sum;
        res.children.push(child);
        pos = next_start_pos;
    }

    for i in 0..meta_count {
        sum += input[pos];
        res.metadata.push(input[pos]);
        pos += 1;
    }
    (res, pos, sum)
}

fn main() {
    let input: Vec<u32> = {
        let mut input = String::new();
        let path = format!("./input/{}", "day08.txt");
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut input).unwrap();
        input
            .split(' ')
            .map(|s| s.parse::<u32>().unwrap())
            .collect()
    };

    // q1
    let (root, _, sum) = parse_node(&input, 0);
    //    println!("{:?}", root);
    println!("result of q01 is {}", sum);
}
