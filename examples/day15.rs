use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::fs::File;

type Coordinate = (usize, usize);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Unit {
    pos: Coordinate,
    hp: u8,
    power: u8,
    faction: char,
}

fn manhattan(c1: &(usize, usize), c2: &(usize, usize)) -> usize {
    let p = (c1.0 as i32 - c2.0 as i32).abs() +
    (c1.1 as i32 - c2.1 as i32).abs();
    p as usize
}

impl Default for Unit {
    fn default() -> Self {
        Unit {
            pos: (0, 0),
            hp: 200,
            power: 3,
            faction: 'E',
        }
    }
}

impl Unit {
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
}

#[derive(Default)]
pub struct Day15 {
    walls: Vec<Vec<bool>>,
    units: Vec<Unit>,
    alive: [usize; 2],
}

impl Day15 {
    pub fn new() -> Self {
        Default::default()
    }

    fn read_input(&mut self, input: &mut Read) {
        let reader = BufReader::new(input);

        for (y, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let mut current = vec![false; line.len()];

            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => { current[x] = true; }
                    'E' | 'G' => {
                        self.units.push(Unit {
                            pos: (y, x),
                            faction: c,
                            ..Default::default()
                        });
                        self.alive[if c == 'E' { 0 } else { 1 }] += 1;
                    }
                    '.' => {}
                    _ => panic!("Invalid tile {}!", c),
                }
            }
            self.walls.push(current);
        }
    }

    fn get_movement(&self, unit: usize) -> Option<Coordinate> {
        let initial = self.units[unit].pos;
        let faction = self.units[unit].faction;

        let enemy_positions: HashSet<Coordinate> = self.units.iter()
            .filter(|x| x.faction != faction && x.is_alive())
            .map(|x| x.pos).collect();

        let all_positions: HashSet<Coordinate> = self.units.iter()
            .filter(|x| x.is_alive())
            .map(|x| x.pos).collect();

        let mut todo = BinaryHeap::new();
        let mut prev: HashMap<Coordinate, Coordinate> = HashMap::new();

        todo.push(Reverse((0, initial)));

        while let Some(Reverse((d, (y, x)))) = todo.pop() {
            let next = [
                (y - 1, x),
                (y, x - 1),
                (y, x + 1),
                (y + 1, x),
            ];

            for pos in &next {
                if !prev.contains_key(pos) && !self.walls[pos.0][pos.1] {
                    if enemy_positions.contains(pos) {
                        return prev.remove(&(y, x));
                    } else if !all_positions.contains(pos) {
                        ;
                        let prev_step = *prev.get(&(y, x)).unwrap_or(pos);
                        prev.insert(*pos, prev_step);
                        todo.push(Reverse((d + 1, *pos)));
                    }
                }
            }
        }
        None
    }

    fn get_attack(&self, unit: usize) -> Option<usize> {
        let initial = self.units[unit].pos;
        let faction = self.units[unit].faction;

        let to_attack = self.units.iter()
            .enumerate()
            .filter(|(_, x)| x.faction != faction && x.is_alive())
            .filter(|(_, x)| manhattan(&x.pos, &initial) == 1)
            .min_by(|&(_, a), &(_, b)| a.hp.cmp(&b.hp).then(a.pos.cmp(&b.pos)));

        if let Some((index, _)) = to_attack {
            Some(index)
        } else {
            None
        }
    }

    fn simulate(&mut self) -> bool {
        self.units.sort_unstable();
        for i in 0..self.units.len() {
            if !self.units[i].is_alive() {
                continue;
            }

            if self.alive[0] == 0 || self.alive[1] == 0 {
                return false;
            }

            if let Some(new_pos) = self.get_movement(i) {
                self.units[i].pos = new_pos;
            }

            if let Some(target) = self.get_attack(i) {
                let power = self.units[i].power;
                let target = &mut self.units[target];
                target.hp = target.hp.saturating_sub(power);

                if target.hp == 0 {
                    match target.faction {
                        'E' => { self.alive[0] -= 1 }
                        'G' => { self.alive[1] -= 1 }
                        _ => panic!(),
                    };
                }
            }
        }

        true
    }

    #[allow(dead_code)]
    fn print(&self) {
        let positions: HashMap<_, _> = self.units.iter()
            .filter(|x| x.is_alive())
            .map(|x| (x.pos, x))
            .collect();

        for y in 0..self.walls.len() {
            let mut buf = String::new();
            let mut unit_buf = String::new();

            for x in 0..self.walls[y].len() {
                if let Some(unit) = positions.get(&(y, x)) {
                    buf.push(unit.faction);

                    unit_buf += &format!("    {}({})", unit.faction, unit.hp);
                } else if self.walls[y][x] {
                    buf.push('#');
                } else {
                    buf.push('.');
                }
            }
            println!("{}{}", buf, unit_buf);
        }
    }

    fn return_score(&self, rounds: usize) -> String {
        let result: usize = rounds * self.units.iter().map(|x| x.hp as usize)
            .sum::<usize>();
        result.to_string()
    }

    fn set_elf_power(&mut self, power: u8) {
        for unit in self.units.iter_mut() {
            if unit.faction == 'E' {
                unit.power = power;
            }
        }
    }



    fn part1(&mut self, input: &mut Read) -> String {
        self.read_input(input);
        let mut rounds = 0;
        while self.simulate() {
            rounds += 1;
        }
        self.return_score(rounds)
    }

    fn part2(&mut self, input: &mut Read) -> String {
        self.read_input(input);
        let backup = self.units.clone();
        let starting_elves = self.alive[0];
        let starting_goblins = self.alive[1];

        let mut power = 4;

        loop {
            self.units = backup.clone();
            self.alive[0] = starting_elves;
            self.alive[1] = starting_goblins;

            self.set_elf_power(power);
            let mut rounds = 0;
            while self.simulate() {
                rounds += 1;
            }

            if self.alive[0] == starting_elves {
                return self.return_score(rounds);
            }

            let to_kill = 200 / power;
            while to_kill == 200 / power {
                power += 1;
            }
        }
    }
}


fn main() {

    let path = format!("./input/{}", "day15.txt");
    let mut read = File::open(path).unwrap();

    let mut instance = Day15::new();

    let q1 = instance.part2(&mut read);
    println!("{}", q1);

//    instance.part2(&mut read);

}
