use std::fs::File;
use std::io::{BufRead, BufReader};

fn tick(reg: &mut [usize; 6], p_idx: usize, instructions: &Vec<(String, usize, usize, usize)>) {
    let cmd = &instructions[reg[p_idx]];
    let opcode = &cmd.0[..];
    match opcode {
        "addr" => reg[cmd.3] = reg[cmd.1] + reg[cmd.2],
        "addi" => reg[cmd.3] = reg[cmd.1] + cmd.2,
        "mulr" => reg[cmd.3] = reg[cmd.1] * reg[cmd.2],
        "muli" => reg[cmd.3] = reg[cmd.1] * cmd.2,
        "setr" => reg[cmd.3] = reg[cmd.1],
        "seti" => reg[cmd.3] = cmd.1,
        "gtir" => reg[cmd.3] = if cmd.1 > reg[cmd.2] { 1 } else { 0 },
        "gtri" => reg[cmd.3] = if reg[cmd.1] > cmd.2 { 1 } else { 0 },
        "gtrr" => reg[cmd.3] = if reg[cmd.1] > reg[cmd.2] { 1 } else { 0 },
        "eqir" => reg[cmd.3] = if cmd.1 == reg[cmd.2] { 1 } else { 0 },
        "eqri" => reg[cmd.3] = if reg[cmd.1] == cmd.2 { 1 } else { 0 },
        "eqrr" => reg[cmd.3] = if reg[cmd.1] == reg[cmd.2] { 1 } else { 0 },
        _ => unreachable!(),
    };
    reg[p_idx] += 1;
    ()
}

fn parse_line(line: String) -> (String, usize, usize, usize) {
    let units = line.split(" ").collect::<Vec<&str>>();
    (
        units[0].to_string(),
        units[1].parse::<usize>().unwrap(),
        units[2].parse::<usize>().unwrap(),
        units[3].parse::<usize>().unwrap(),
    )
}

fn main() {
    let path = format!("./input/{}", "day19.txt");
    let mut iter = BufReader::new(File::open(path).unwrap()).lines();
    let p_idx: usize = iter.next().unwrap().unwrap()[4..].parse::<usize>().unwrap();
    let instructions: Vec<(String, usize, usize, usize)> = iter
        .map(|l| l.expect("Could not parse line"))
        .map(|line| parse_line(line))
        .collect();

    let mut register = [0usize; 6];
    while register[p_idx] < instructions.len() {
        tick(&mut register, p_idx, &instructions);
    }
    println!("result of q01 is {}", register[0]);
}
