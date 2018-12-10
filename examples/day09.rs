use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

fn rotate_backward(deque: &mut VecDeque<usize>, by: usize) {
    (0..by).for_each(|_| {
        let tmp = deque.pop_back().unwrap();
        deque.push_front(tmp);
    });
}

fn rotate_forward(deque: &mut VecDeque<usize>, by: usize) {
    (0..by).for_each(|_| {
        let tmp = deque.pop_front().unwrap();
        deque.push_back(tmp);
    });
}

fn solution(players: usize, last_marble: usize) -> usize {
    let mut score_counter = vec![0usize; players];
    let mut ring_chain: VecDeque<usize> = VecDeque::with_capacity((last_marble + 1) as usize);
    ring_chain.push_front(0);

    for i in 1..=last_marble {
        if i % 23 == 0 {
            rotate_backward(&mut ring_chain, 7);
            score_counter[i % players] += i + ring_chain.pop_front().unwrap();
        } else {
            rotate_forward(&mut ring_chain, 2);
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
