use std::iter::FromIterator;

fn main() {
    let input = 846601;
    let mut vec: Vec<u8> = Vec::with_capacity(50_000_000);
    vec.push(3);
    vec.push(7);
    let mut i_a = 0usize;
    let mut i_b = 1usize;

    while vec.len() < 50_000_000 {
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

    println!(
        "result of q01 is {}",
        String::from_iter(
            vec[input..input + 10]
                .iter()
                .map(|&u| char::from(u + '0' as u8))
                .collect::<Vec<char>>()
        )
    );

    let line = String::from_iter(
        vec.iter()
            .map(|&u| char::from(u + '0' as u8))
            .collect::<Vec<char>>(),
    );
    let q2 = line.find(&input.to_string()).unwrap();
    println!("result of q02 is {}", q2);
}
