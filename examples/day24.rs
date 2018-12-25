#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate z3;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;
use std::collections::HashSet;
use AttackType::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum AttackType {
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}

#[derive(Debug, Clone)]
struct Group {
    gid: usize, // global unique id
    team: usize,
    units: i32,
    hp: i32,
    damage: i32,
    att_type: AttackType,
    initiative: i32,
    weak: Vec<AttackType>,
    immune: Vec<AttackType>,
}

impl Group {
    fn get_ep(&self) -> i32 {
        self.units * self.damage
    }

    fn attack(&self, target: &mut Group) {
        if target.units <= 0 {
            return;
        }

        let damage_will_apply = if target.weak.contains(&self.att_type) {
            2 * self.get_ep()
        } else if target.immune.contains(&self.att_type) {
            0
        } else {
            self.get_ep()
        };
        // do damage
        let killed_units = damage_will_apply / target.hp;
        target.units -= killed_units;
    }

    fn simulate_damage(&self, target: &Group) -> i32 {
        if self.units <= 0 {
            return 0;
        }

        if target.weak.contains(&self.att_type) {
            2 * self.get_ep()
        } else if target.immune.contains(&self.att_type) {
            0
        } else {
            self.get_ep()
        }
    }
}

fn solution(groups: &mut Vec<Group>) -> (usize, i32) {
    loop {
        // target selection
        groups.sort_by(|ref a, ref b| {
            b.get_ep()
                .cmp(&a.get_ep())
                .then(b.initiative.cmp(&a.initiative))
        });

        let mut source_target_map: HashMap<usize, usize> = HashMap::new();
        let mut have_been_selected = HashSet::new();
        for i in 0..groups.len() {
            let g = &groups[i];

            let mut tmp: Vec<(&Group, i32)> = groups
                .iter()
                .filter(|&tar| g.team != tar.team)
                .map(|tar| (tar, g.simulate_damage(tar)))
                .collect();
            tmp.sort_by(|&t1, &t2| {
                t2.1.cmp(&t1.1)
                    .then(t2.0.get_ep().cmp(&t1.0.get_ep()))
                    .then(t2.0.initiative.cmp(&t1.0.initiative))
            });

            tmp = tmp
                .into_iter()
                .filter(|&t| t.1 != 0 && !have_been_selected.contains(&t.0.gid))
                .collect();

            if let Some(selected) = tmp.first() {
                if selected.1 != 0 {
                    source_target_map.insert(g.gid, selected.0.gid);
                    have_been_selected.insert(selected.0.gid);
                }
            }
        }

        // attack
        groups.sort_by(|ref a, ref b| b.initiative.cmp(&a.initiative));
        let mut no_unit_lost_in_one_loop = true;
        for i in 0..groups.len() {
            let cur_gid = groups[i].gid;
            match source_target_map.get(&cur_gid) {
                Some(&tar_id) => {
                    let dmg = {
                        let tar_g = groups.iter().find(|p| p.gid == tar_id).unwrap();
                        groups[i].simulate_damage(tar_g)
                    };

                    let mut tar_g = groups.iter_mut().find(|p| p.gid == tar_id).unwrap();
                    if dmg / tar_g.hp > 0 {
                        no_unit_lost_in_one_loop = false;
                    }
                    tar_g.units -= dmg / tar_g.hp;
                }
                _ => continue,
            }
        }
        if no_unit_lost_in_one_loop {
            return (0, -1);
        }

        // clear destroyed groups
        groups.retain(|g| g.units > 0);

        // check whole army destroyed
        let team0_count = groups.iter().filter(|&g| g.team == 0).count();
        let team1_count = groups.iter().filter(|&g| g.team == 1).count();
        if team0_count * team1_count == 0 {
            let units_left: i32 = groups.iter().map(|ref g| g.units).sum();
            return (if team0_count > 0 { 0 } else { 1 }, units_left);
        }
    }
}

fn main() {
    let path = format!("./input/{}", "day24.txt");

    let inputs: Vec<String> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let groups = parse_all(&inputs);

    let mut groups_q1 = groups.clone();
    let q1_res = solution(&mut groups_q1);
    println!("result of q01 is {}", q1_res.1);

    let mut boost = 1;
    loop {
        let mut groups_q2 = groups.clone();
        groups_q2.iter_mut().for_each(|g| {
            if g.team == 0 {
                g.damage += boost
            }
        });
        let res = solution(&mut groups_q2);
        if res.0 == 0 && res.1 != -1 {
            println!("result of q02 is {} {}", res.1, boost);
            break;
        }
        boost += 1;
    }
}

fn parse_all(inputs: &Vec<String>) -> Vec<Group> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<units>\d+) units each with (?P<hp>\d+) hit points (?:(?P<optional>.*))with an attack that does (?P<damage>\d+) (?P<att_type>[a-z]+) damage at initiative (?P<initiative>\d+)").unwrap();
    }

    let mut team = 0;
    let mut res = Vec::new();

    for line in inputs {
        if line == "Immune System:" || line.len() <= 1 {
            continue;
        } else if line == "Infection:" {
            team = 1;
        } else {
            let caps = RE.captures(&line).unwrap();
            let units = caps["units"].parse::<i32>().unwrap();
            let hp = caps["hp"].parse::<i32>().unwrap();
            let damage = caps["damage"].parse::<i32>().unwrap();
            let initiative = caps["initiative"].parse::<i32>().unwrap();

            let mut weak = Vec::new();
            let mut immune = Vec::new();
            let optional = &caps["optional"];
            if optional.len() > 2 {
                optional
                    .split("; ")
                    .collect::<Vec<&str>>()
                    .iter()
                    .for_each(|part| {
                        if part.contains("immune to") {
                            silly_add(part, &mut immune);
                        } else if part.contains("weak to") {
                            silly_add(part, &mut weak);
                        }
                    });
            }

            let att_type = silly_transform(&caps["att_type"]);
            let gid = res.len();
            res.push(Group {
                gid,
                team,
                units,
                hp,
                damage,
                att_type,
                weak,
                immune,
                initiative,
            });
        }
    }
    //    println!("{:?}", res);
    res
}

fn silly_add(part: &str, collection: &mut Vec<AttackType>) {
    for word in &["slashing", "bludgeoning", "radiation", "fire", "cold"] {
        if part.contains(word) {
            collection.push(silly_transform(word));
        }
    }
}

fn silly_transform(word: &str) -> AttackType {
    match word {
        "slashing" => Slashing,
        "bludgeoning" => Bludgeoning,
        "radiation" => Radiation,
        "fire" => Fire,
        _ => Cold,
    }
}
