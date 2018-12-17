use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Assertion {
    before: [usize; 4],
    cmd: (usize, usize, usize, usize),
    after: [usize; 4],
}

impl Assertion {
    fn try_assert(&self, index: usize) -> bool {
        let gen = execute(&self.before, self.cmd, index);
        gen[0] == self.after[0]
            && gen[1] == self.after[1]
            && gen[2] == self.after[2]
            && gen[3] == self.after[3]
    }
}

fn execute(registers: &[usize; 4], cmd: (usize, usize, usize, usize), index: usize) -> [usize; 4] {
    let mut gen: [usize; 4] = Default::default();
    gen.copy_from_slice(registers);
    let be = registers;
    match index {
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
    gen
}

fn main() {
    let path = format!("./input/{}", "day16_q1.txt");
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

    // gen opcode->Set(indexes) mapping
    let mut opcode_mapping: Vec<HashSet<usize>> = vec![HashSet::new(); 16];

    asserts.iter().for_each(|ref ass| {
        let opcode = ass.cmd.0;
        let mut index_set: HashSet<usize> = HashSet::new();
        (0..=15usize).for_each(|idx| {
            if ass.try_assert(idx) {
                index_set.insert(idx);
            }
        });
        let un_init = opcode_mapping[opcode].is_empty();
        if un_init {
            opcode_mapping[opcode] = index_set;
        } else {
            opcode_mapping[opcode] = opcode_mapping[opcode]
                .intersection(&index_set)
                .map(|&x| x)
                .collect::<HashSet<usize>>();
        }
    });
    println!("DEBUG before deduction {:?}", opcode_mapping);

    // repeat deduction until every Set.len() == 1
    let mut handled: HashSet<usize> = HashSet::new();
    while opcode_mapping.iter().any(|set| set.len() != 1) {
        for set in opcode_mapping.iter_mut() {
            if set.len() > 1 {
                handled.iter().for_each(|num| {
                    set.remove(&num);
                })
            } else {
                handled.insert(*set.iter().next().unwrap());
            }
        }
    }
    // deduction & normalize
    let opcode_mapping = opcode_mapping
        .into_iter()
        .map(|set| set.into_iter().next().unwrap())
        .collect::<Vec<usize>>();
    println!("DEBUG after deduction {:?}", opcode_mapping);

    // parsing q2 input
    let path = format!("./input/{}", "day16_q2.txt");
    let instructions: Vec<Vec<usize>> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|line| {
            line.split(' ')
                .map(|u| u.parse::<usize>().expect(""))
                .collect()
        })
        .collect();

    let mut register: [usize; 4] = [0usize; 4];
    for ins in instructions {
        register = execute(
            &register,
            (ins[0], ins[1], ins[2], ins[3]),
            opcode_mapping[ins[0]],
        );
    }
    println!("result of q02 is {}", register[0]);
}
