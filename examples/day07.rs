use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX_STEPS: usize = 26;
const WORKER_NUMBERS: usize = 5;
const BASE_PROCESS_TIME: u32 = 60;

//const MAX_STEPS: usize = 6;
//const WORKER_NUMBERS: usize = 2;
//const BASE_PROCESS_TIME: u32 = 0;

#[derive(Debug, Clone)]
struct Step {
    q1_handled: bool,
    // after done the step, mark it as true
    q2_handled: bool,
    // after worker handled the step, mark it as true
    from: Vec<usize>,
    to: Vec<usize>,
}

fn char2usize(c: char) -> usize {
    c as usize - 'A' as usize
}

fn parse_all(vec: Vec<String>) -> Vec<Step> {
    let mut dots: Vec<Step> = vec![
        Step {
            q1_handled: false,
            q2_handled: false,
            from: Vec::new(),
            to: Vec::new(),
        };
        MAX_STEPS
    ];

    vec.iter().for_each(|line| {
        let words = line.split(' ').collect::<Vec<&str>>();
        let start = words[1].parse::<char>().unwrap();
        let end = words[7].parse::<char>().unwrap();
        dots[char2usize(start)].to.push(char2usize(end));
        dots[char2usize(end)].from.push(char2usize(start));
    });

    dots
}

fn q1(steps: &mut Vec<Step>) {
    let mut result = String::new();
    let mut s_queue = HashSet::new();
    for i in 0..MAX_STEPS {
        if steps[i].from.is_empty() {
            s_queue.insert(i);
        }
    }

    while !s_queue.is_empty() {
        let least_available_step_id: usize = {
            // target to least index && available(all from nodes handled)
            let res = *s_queue
                .iter()
                .filter(|&index| {
                    steps[*index]
                        .from
                        .iter()
                        .filter(|&idx| !steps[*idx].q1_handled)
                        .collect::<Vec<&usize>>()
                        .is_empty()
                })
                .min()
                .unwrap();

            // delete from s_queue(pop out)
            s_queue.retain(|&x| x != res);
            res
        };

        // mark as handled
        steps[least_available_step_id].q1_handled = true;

        // add all 'to' element to s_queue
        steps[least_available_step_id].to.iter().for_each(|&id| {
            s_queue.insert(id);
        });

        // concat step name to final result
        result.push((least_available_step_id as u8 + 'A' as u8) as char);
    }

    println!("result of q01 is {}", result);
}

#[derive(Debug, Clone, PartialEq)]
enum WorkerState {
    IDLE,
    OnProcess(usize /*stepId*/, u32 /*seconds left*/),
}

#[derive(Debug, Clone)]
struct Worker {
    state: WorkerState,
}

impl Worker {
    fn tick(&mut self) -> Option<usize> {
        match self.state {
            WorkerState::OnProcess(step_id, left) => {
                if left == 1 {
                    self.state = WorkerState::IDLE;
                    Some(step_id)
                } else {
                    self.state = WorkerState::OnProcess(step_id, left - 1);
                    None
                }
            }
            WorkerState::IDLE => None,
        }
    }
}

fn is_all_workers_idle(workers: &Vec<Worker>) -> bool {
    workers
        .iter()
        .fold(true, |b, ref worker| b && worker.state == WorkerState::IDLE)
}

fn q2(steps: &mut Vec<Step>) {
    let mut seconds: i32 = 0;
    let mut task_queue = HashSet::new();
    for i in 0..MAX_STEPS {
        if steps[i].from.is_empty() {
            task_queue.insert(i);
        }
    }

    let mut workers = vec![
        Worker {
            state: WorkerState::IDLE
        };
        WORKER_NUMBERS
    ];

    while !task_queue.is_empty() || !is_all_workers_idle(&workers) {
        workers.iter_mut().for_each(|wk| {
            if let Some(finished_step_id) = wk.tick() {
                steps[finished_step_id].q2_handled = true;

                // add step's 'to' to task_queue
                steps[finished_step_id].to.iter().for_each(|&id| {
                    task_queue.insert(id);
                });
            }
        });

        workers.iter_mut().for_each(|wk| {
            if wk.state == WorkerState::IDLE {
                // fetch a leastId && available step. (available == all from steps are q2_handled)
                // may not existed
                let least_available_step_id_option: Option<usize> = task_queue
                    .iter()
                    .filter(|&index| {
                        steps[*index]
                            .from
                            .iter()
                            .filter(|&idx| !steps[*idx].q2_handled)
                            .collect::<Vec<&usize>>()
                            .is_empty()
                    })
                    .min()
                    .map(|a| a.to_owned());

                if let Some(least_available_step_id) = least_available_step_id_option {
                    // delete from task_queue(pop out)
                    task_queue.retain(|&x| x != least_available_step_id);
                    // assign step to worker
                    let step_processing_time: u32 =
                        least_available_step_id as u32 + 1 + BASE_PROCESS_TIME;
                    wk.state =
                        WorkerState::OnProcess(least_available_step_id, step_processing_time);
                }
            }
        });
        seconds += 1;
    }
    println!("result of q02 is {}", seconds - 1);
}

fn main() {
    let path = format!("./input/{}", "day07.txt");

    let vec: Vec<String> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut steps = parse_all(vec);
    //        steps.iter().for_each(|x| println!("{:?}", x));
    q1(&mut steps);
    q2(&mut steps);
}
