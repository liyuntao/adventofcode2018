use std::fs::File;
use std::io::Read;

fn solution(players: usize, last_marble: u32) -> u32 {
    let mut cur_center_idx: usize = 0;
    let mut cur_player_id = 0;
    let mut score_counter = vec![0u32; players];
    let mut chain: Vec<u32> = Vec::with_capacity((last_marble + 1) as usize);
    chain.push(0);

    for i in 1..=last_marble {
        if i % 23 == 0 {
            score_counter[cur_player_id] += i;
            cur_center_idx = {
                let mut tmp = cur_center_idx as i32;
                tmp -= 7;
                if tmp < 0 {
                    (tmp + chain.len() as i32) as usize
                } else {
                    tmp as usize
                }
            };
            score_counter[cur_player_id] += chain[cur_center_idx];
            chain.remove(cur_center_idx);

        //            println!("{:?}", score_counter);
        } else {
            // find target index (right shift then add 1 then insert)
            cur_center_idx = (cur_center_idx + 1) % chain.len() + 1;
            chain.insert(cur_center_idx, i);
        }
        //        println!("[{}]  {:?}", cur_player_id + 1, chain);
        cur_player_id = (cur_player_id + 1) % players
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
            vec[6].parse::<u32>().unwrap(),
        )
    };

    assert_eq!(8317, solution(10, 1618));
    assert_eq!(146373, solution(13, 7999));
    assert_eq!(2764, solution(17, 1104));

    let res_q1 = solution(players, last_marble);
    println!("result of q01 is {}", res_q1);

    // TODO q2 may need LinkedList with cursor implemented...
}
