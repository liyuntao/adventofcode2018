use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
    value: u32,
}

fn parse_node(input: &Vec<u32>, start_pos: usize) -> (Node, usize, u32) {
    let mut pos = start_pos;
    let mut meta_sum = 0;

    let child_count = input[pos];
    pos += 1;
    let meta_count = input[pos];
    pos += 1;

    let mut res = Node {
        children: Vec::new(),
        metadata: Vec::new(),
        value: 0,
    };

    for _i in 0..child_count {
        let (child, next_start_pos, child_sum) = parse_node(input, pos);
        meta_sum += child_sum;
        res.children.push(child);
        pos = next_start_pos;
    }

    for _i in 0..meta_count {
        meta_sum += input[pos];
        res.metadata.push(input[pos]);

        if res.children.is_empty() {
            res.value += input[pos];
        } else {
            let index = input[pos] as usize;
            if index > 0 && index <= res.children.len() {
                res.value += res.children[index - 1].value;
            }
        }
        pos += 1;
    }
    (res, pos, meta_sum)
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

    let (root, _, meta_sum) = parse_node(&input, 0);
    println!("result of q01 is {}", meta_sum);
    println!("result of q02 is {}", root.value);
}
