use std::fs::File;
use std::io::{BufRead, BufReader};

struct Coordinate {
    x: i32,
    y: i32,
    reach_infinite: bool,
    area_counter: u32,
}

impl Coordinate {
    fn manhattan_distance(&self, x: i32, y: i32) -> i32 {
        (x - self.x).abs() + (y - self.y).abs()
    }
}

fn is_fetch_infinite(x: i32, y: i32, min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> bool {
    x == min_x || x == max_x || y == min_y || y == max_y
}

fn parse_line(line: &str) -> Coordinate {
    let res = line.split(", ").collect::<Vec<&str>>();
    Coordinate {
        x: res[0].parse::<i32>().unwrap(),
        y: res[1].parse::<i32>().unwrap(),
        reach_infinite: false,
        area_counter: 0,
    }
}

fn main() {
    let path = format!("./input/{}", "day06.txt");

    let vec: Vec<String> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    // q1
    let mut dots: Vec<Coordinate> = vec.iter().map(|line| parse_line(line)).collect();
    let min_x: i32 = dots.iter().map(|dot| dot.x).min().unwrap();
    let min_y: i32 = dots.iter().map(|dot| dot.y).min().unwrap();
    let max_x: i32 = dots.iter().map(|dot| dot.x).max().unwrap();
    let max_y: i32 = dots.iter().map(|dot| dot.y).max().unwrap();

    for i in min_x..=max_x {
        for j in min_y..=max_y {
            let manh_dis_vec: Vec<i32> = dots
                .iter()
                .map(|dot| dot.manhattan_distance(i, j))
                .collect();
            let min_dis = *manh_dis_vec.iter().min().unwrap();
            if manh_dis_vec
                .iter()
                .filter(|&&distance| distance == min_dis)
                .count()
                == 1
            {
                let coor_index = manh_dis_vec
                    .iter()
                    .position(|&distance| distance == min_dis)
                    .unwrap();
                dots[coor_index].area_counter += 1;

                if is_fetch_infinite(i, j, min_x, min_y, max_x, max_y) {
                    dots[coor_index].reach_infinite = true;
                }
            }
        }
    }

    println!(
        "result of q01 is {}",
        dots.iter()
            .filter(|dot| !dot.reach_infinite)
            .map(|dot| dot.area_counter)
            .max()
            .unwrap()
    );
}
