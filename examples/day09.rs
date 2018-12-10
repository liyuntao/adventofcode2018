use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

fn solution(players: usize, last_marble: usize) -> usize {
    let mut score_counter = vec![0usize; players];
    let mut ring_chain: VecDeque<usize> = VecDeque::with_capacity((last_marble + 1) as usize);
    ring_chain.push_front(0);

    for i in 1..=last_marble {
        if i % 23 == 0 {
            // rotate of 7 behind + delete
            (0..7).for_each(|_| {
                let tmp = ring_chain.pop_back().unwrap();
                ring_chain.push_front(tmp);
            });
            score_counter[i % players] += i + ring_chain.pop_front().unwrap();
        } else {
            (0..2).for_each(|_| {
                let tmp = ring_chain.pop_front().unwrap();
                ring_chain.push_back(tmp);
            });
            ring_chain.push_front(i);
        }
    }

    score_counter.into_iter().max().unwrap()
}

fn main() {
    let (players, last_marble) = {
        let mut input = String::new();
        let path = format!("./input/{}", "day09.txt");
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut input).unwrap();
        let vec: Vec<&str> = input.split(' ').collect();
        (
            vec[0].parse::<usize>().unwrap(),
            vec[6].parse::<usize>().unwrap(),
        )
    };

    assert_eq!(8317, solution(10, 1618));
    assert_eq!(146373, solution(13, 7999));
    assert_eq!(2764, solution(17, 1104));

    let res_q1 = solution(players, last_marble);
    println!("result of q01 is {}", res_q1);

    let res_q2 = solution(players, last_marble * 100);
    println!("result of q02 is {}", res_q2);
}
