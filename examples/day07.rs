use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX_STEPS: usize = 26;
//const MAX_STEPS: usize = 6;

#[derive(Debug, Clone)]
struct Step {
    handled: bool,
    from: Vec<usize>,
    to: Vec<usize>,
}

fn char2usize(c: char) -> usize {
    c as usize - 'A' as usize
}

fn parse_all(vec: Vec<String>) -> Vec<Step> {
    let mut dots: Vec<Step> = vec![
        Step {
            handled: false,
            from: Vec::new(),
            to: Vec::new()
        };
        MAX_STEPS
    ];

    vec.iter().for_each(|line| {
        let words = line.split(' ').collect::<Vec<&str>>();
        let start = words[1].parse::<char>().unwrap();
        let end = words[7].parse::<char>().unwrap();
        dots[char2usize(start)].to.push(char2usize(end));
        dots[char2usize(end)].from.push(char2usize(start));
    });

    dots
}

fn main() {
    let path = format!("./input/{}", "day07.txt");

    let vec: Vec<String> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut steps = parse_all(vec);
    let mut result = String::new();


    // q1
//    steps.iter().for_each(|x| println!("{:?}", x));
    let mut s_queue = HashSet::new();
    for i in 0..MAX_STEPS {
        if steps[i].from.is_empty() {
            s_queue.insert(i);
        }
    }

    while !s_queue.is_empty() {
        let least_available_step_id: usize = {
            // target to least index && available(all from nodes handled)
            let res = *s_queue
                .iter()
                .filter(|&index| {
                    steps[*index]
                        .from
                        .iter()
                        .filter(|&idx| !steps[*idx].handled)
                        .collect::<Vec<&usize>>()
                        .is_empty()
                })
                .min()
                .unwrap();

            // delete from s_queue(pop out)
            s_queue.retain(|&x| x != res);
            res
        };

        // mark as handled
        steps[least_available_step_id].handled = true;

        // add all 'to' element to s_queue
        steps[least_available_step_id].to.iter().for_each(|&id| {
            s_queue.insert(id);
        });

        // concat step name to final result
        result.push((least_available_step_id as u8 + 'A' as u8) as char);
    }

    println!("result of q01 is {}", result);
}
