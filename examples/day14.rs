const GEN_SIZE: usize = 50_000_000;

fn solution(input: usize) {
    let mut vec: Vec<u8> = Vec::with_capacity(GEN_SIZE);
    vec.push(3);
    vec.push(7);
    let mut i_a = 0usize;
    let mut i_b = 1usize;

    while vec.len() < GEN_SIZE {
        // make 1 or 2 recipe
        let score = vec[i_a] + vec[i_b];
        if score >= 10 {
            vec.push(1);
            vec.push(score % 10);
        } else {
            vec.push(score);
        }
        // move index
        i_a = (i_a + 1 + vec[i_a] as usize) % vec.len();
        i_b = (i_b + 1 + vec[i_b] as usize) % vec.len();
    }

    let q1_str: String = vec[input..input + 10]
        .iter()
        .map(|&u| char::from(u + '0' as u8))
        .collect();
    println!("result of q01 is {}", q1_str);

    let line: String = vec.iter().map(|&u| char::from(u + '0' as u8)).collect();
    let q2 = line.find(&input.to_string()).unwrap();
    println!("result of q02 is {}", q2);
}

fn main() {
    solution(846601);
}
