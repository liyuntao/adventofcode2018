use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Assertion {
    before: [usize; 4],
    cmd: (usize, usize, usize, usize),
    after: [usize; 4],
}

impl Assertion {
    fn try_assert(&self, id: usize) -> bool {
        let mut gen: [usize; 4] = Default::default();
        gen.copy_from_slice(&self.after);
        let be = &self.before;
        let cmd = &self.cmd;

        match id {
            0 => gen[cmd.3] = be[cmd.1] + be[cmd.2],
            1 => gen[cmd.3] = be[cmd.1] + cmd.2,
            2 => gen[cmd.3] = be[cmd.1] * be[cmd.2],
            3 => gen[cmd.3] = be[cmd.1] * cmd.2,
            4 => gen[cmd.3] = be[cmd.1] & be[cmd.2],
            5 => gen[cmd.3] = be[cmd.1] & cmd.2,
            6 => gen[cmd.3] = be[cmd.1] | be[cmd.2],
            7 => gen[cmd.3] = be[cmd.1] | cmd.2,
            8 => gen[cmd.3] = be[cmd.1],
            9 => gen[cmd.3] = cmd.1,
            10 => gen[cmd.3] = if cmd.1 > be[cmd.2] { 1 } else { 0 },
            11 => gen[cmd.3] = if be[cmd.1] > cmd.2 { 1 } else { 0 },
            12 => gen[cmd.3] = if be[cmd.1] > be[cmd.2] { 1 } else { 0 },
            13 => gen[cmd.3] = if cmd.1 == be[cmd.2] { 1 } else { 0 },
            14 => gen[cmd.3] = if be[cmd.1] == cmd.2 { 1 } else { 0 },
            15 => gen[cmd.3] = if be[cmd.1] == be[cmd.2] { 1 } else { 0 },
            _ => {}
        };

        gen[0] == self.after[0]
            && gen[1] == self.after[1]
            && gen[2] == self.after[2]
            && gen[3] == self.after[3]
    }
}

fn main() {
    let path = format!("./input/{}", "day15_q1.txt");

    let vec: Vec<String> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let asserts: Vec<Assertion> = vec
        .chunks(4)
        .map(|s| {
            let befores: Vec<usize> = s[0][9..19]
                .split(", ")
                .map(|u| u.parse::<usize>().expect(""))
                .collect();
            let cmds: Vec<usize> = s[1]
                .split(' ')
                .map(|u| u.parse::<usize>().expect(""))
                .collect();
            let afters: Vec<usize> = s[2][9..19]
                .split(", ")
                .map(|u| u.parse::<usize>().expect(""))
                .collect();
            let mut before: [usize; 4] = Default::default();
            before.copy_from_slice(&befores[0..4]);
            let mut after: [usize; 4] = Default::default();
            after.copy_from_slice(&afters[0..4]);

            Assertion {
                before,
                cmd: (cmds[0], cmds[1], cmds[2], cmds[3]),
                after,
            }
        })
        .collect();

    let q1 = asserts
        .iter()
        .map(|ass| {
            (0..=15)
                .map(|id| ass.try_assert(id))
                .filter(|&b| b)
                .collect::<Vec<bool>>()
        })
        .filter(|ref res_vec| res_vec.len() >= 3)
        .count();

    println!("result of q01 is {}", q1);
}
