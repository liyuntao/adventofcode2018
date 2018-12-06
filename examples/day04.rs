extern crate chrono;

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use std::fs::File;
use std::io::{BufRead, BufReader};

use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum StateChange {
    BeginShift(String /*ID*/),
    FallAsleep(u32 /* moment 0-59 */),
    WakeUp(u32 /* moment 0-59 */),
}

#[derive(Debug)]
struct Record {
    date_mark: String,
    datetime: DateTime<Utc>, // basically for sorting
    state: StateChange,
}

fn parse(s: &str) -> Record {
    let dt = Utc.datetime_from_str(&s[1..17], "%Y-%m-%d %H:%M").unwrap();

    let state = if s.contains("begins shift") {
        let id = s[19..].split(' ').collect::<Vec<&str>>()[1];
        StateChange::BeginShift(id.to_string())
    } else if s.contains("falls asleep") {
        StateChange::FallAsleep(dt.minute())
    } else {
        StateChange::WakeUp(dt.minute())
    };

    Record {
        datetime: dt,
        state: state,
        date_mark: gen_date_mark(dt),
    }
}

// if starts with 23:xx, make the date_mark to the next-day
fn gen_date_mark(dt: DateTime<Utc>) -> String {
    let dt = if dt.hour() == 23 {
        dt.clone() + Duration::days(1)
    } else {
        dt
    };

    dt.format("%m-%d").to_string()
}

fn handle_solution(vec: Vec<String>) {
    let mut records = vec
        .iter()
        .map(|input| parse(&input))
        .collect::<Vec<Record>>();
    records.sort_by(|ref a, ref b| a.datetime.cmp(&b.datetime));
    //    records.iter().for_each(|rec| println!("debug {:?}", rec));

    let mut total_sleep_minutes_counter: HashMap<
        String, /*ID*/
        u32,    /*total minutes*/
    > = HashMap::new();
    let mut sleep_moments_marker: HashMap<
        String,   /*ID*/
        Vec<u32>, /*moment fallasleep counter*/
    > = HashMap::new();
    let mut cur_id: &str = "";
    let mut last_fallasleep_moment: u32 = 0;

    // count total_asleep_minutes by ID
    records.iter().for_each(|rec| match rec.state {
        StateChange::BeginShift(ref id) => {
            cur_id = id;
            if !total_sleep_minutes_counter.contains_key(id) {
                total_sleep_minutes_counter.insert(id.to_string(), 0);
                sleep_moments_marker.insert(id.to_string(), vec![0 as u32; 60]);
            }
        }
        StateChange::FallAsleep(m) => last_fallasleep_moment = m,
        StateChange::WakeUp(m) => {
            let count = total_sleep_minutes_counter[cur_id] + m - last_fallasleep_moment;
            total_sleep_minutes_counter.insert(cur_id.to_string(), count);
            for i in last_fallasleep_moment..m {
                let mut marker = sleep_moments_marker.get_mut(cur_id).unwrap();
                marker[i as usize] += 1;
            }
        }
    });

    // q1
    let max_k_v = total_sleep_minutes_counter
        .iter()
        .max_by(|&a, &b| a.1.cmp(b.1))
        .unwrap();
//    println!("{}: \"{}\"", max_k_v.0, max_k_v.1);
//    println!("{:?}", sleep_moments_marker[max_k_v.0]);
//    println!(
//        "max counts fallasleep {}",
//        sleep_moments_marker[max_k_v.0].iter().max().unwrap()
//    );

    let id_i32 = max_k_v.0[1..].parse::<i32>().unwrap();
    let max_minutes = *sleep_moments_marker[max_k_v.0].iter().max().unwrap();
    let max_moment_i32 = sleep_moments_marker[max_k_v.0]
        .iter()
        .position(|&r| r == max_minutes)
        .unwrap() as i32;

    println!("result of q01 is {}", id_i32 * max_moment_i32);

    // q2
    let max_tuple = sleep_moments_marker
        .iter()
        .map(|t| (t.0, t.1.iter().max().unwrap()))
        .max_by(|&t1, &t2| t1.1.cmp(t2.1))
        .unwrap();
    println!("{}: \"{}\"", max_tuple.0, max_tuple.1);
    println!("{:?}", sleep_moments_marker[max_tuple.0]);

    let id02_i32 = max_tuple.0[1..].parse::<i32>().unwrap();
    let max_minutes = *max_tuple.1;
    let max_moment02_i32 = sleep_moments_marker[max_tuple.0]
        .iter()
        .position(|&r| r == max_minutes)
        .unwrap() as i32;

    println!("result of q02 is {}", id02_i32 * max_moment02_i32);
}

fn main() {
    let path = format!("./input/{}", "day04.txt");

    let vec: Vec<String> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    // q1
    handle_solution(vec);
}
