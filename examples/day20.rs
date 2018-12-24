use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

fn solution(input: String) {
    let mut grid: HashMap<(i32, i32), usize> = HashMap::new();
    let mut cur_pos: (i32, i32) = (0, 0);
    grid.insert(cur_pos, 0);
    let mut stack = VecDeque::new();
    for c in input.chars() {
        match c {
            'N' => {
                grid.insert((cur_pos.0, cur_pos.1 - 1), 0);
                cur_pos.1 -= 2;
                grid.insert(cur_pos, usize::max_value());
            }
            'S' => {
                grid.insert((cur_pos.0, cur_pos.1 + 1), 0);
                cur_pos.1 += 2;
                grid.insert(cur_pos, usize::max_value());
            }
            'W' => {
                grid.insert((cur_pos.0 - 1, cur_pos.1), 0);
                cur_pos.0 -= 2;
                grid.insert(cur_pos, usize::max_value());
            }
            'E' => {
                grid.insert((cur_pos.0 + 1, cur_pos.1), 0);
                cur_pos.0 += 2;
                grid.insert(cur_pos, usize::max_value());
            }
            '(' => stack.push_back(cur_pos),
            ')' => cur_pos = stack.pop_back().unwrap(),
            '|' => cur_pos = *stack.back().unwrap(),
            _ => {}
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        let cur_fewest = *grid.get(&cur).unwrap();

        for t in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let next_x = cur.0 + t.0 * 2;
            let next_y = cur.1 + t.1 * 2;

            if grid.contains_key(&(cur.0 + t.0, cur.1 + t.1)) {
                let last_fewest = *grid.get(&(next_x, next_y)).unwrap();
                if cur_fewest + 1 < last_fewest {
                    grid.insert((next_x, next_y), cur_fewest + 1);
                    queue.push_back((next_x, next_y));
                }
            }
        }
    }

    let q1 = grid.iter().max_by(|&e1, &e2| e1.1.cmp(e2.1)).unwrap();
    println!("result of q01 is {}", q1.1);

    let q2 = grid.iter().filter(|&entry| *entry.1 >= 1000).count();
    println!("result of q02 is {}", q2);
}

fn main() {
    let input = {
        let mut input = String::new();
        let path = format!("./input/{}", "day20.txt");
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut input).unwrap();
        input
    };

    solution(input);
}
