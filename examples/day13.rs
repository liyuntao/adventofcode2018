use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn anti_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn shift(&self) -> Turn {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

#[derive(Debug)]
struct Cart {
    x: usize,
    y: usize,
    direction: Direction,
    next_turn: Turn,
}

impl Cart {
    fn new(x: usize, y: usize, c: char) -> Cart {
        Cart {
            x,
            y,
            next_turn: Turn::Left,
            direction: match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                _ => Direction::Right,
            },
        }
    }

    fn tick(&mut self, road: &Vec<Vec<char>>) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }

        match road[self.y][self.x] {
            '\\' => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                }
            }
            '/' => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                }
            }
            '+' => {
                match self.next_turn {
                    Turn::Left => self.direction = self.direction.anti_clockwise(),
                    Turn::Right => self.direction = self.direction.clockwise(),
                    _ => {}
                }
                self.next_turn = self.next_turn.shift();
            }
            _ => {}
        }
        ()
    }
}

fn parse_all(inputs: Vec<String>) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    let max_line_len = inputs.iter().map(|line| line.len()).max().unwrap();
    inputs.iter().for_each(|line| {
        let mut tmp = vec![' '; max_line_len];
        for (i, c) in line.chars().enumerate() {
            tmp[i] = c;
        }
        grid.push(tmp);
    });
    grid
}

fn gen_carts(vec: &mut Vec<Vec<char>>) -> Vec<Cart> {
    let mut carts = Vec::new();
    for y in 0..vec.len() {
        for x in 0..vec[0].len() {
            let c = vec[y][x];
            if c == '^' || c == 'v' || c == '<' || c == '>' {
                carts.push(Cart::new(x, y, c));
            }
        }
    }
    carts
}

fn solution(road: &Vec<Vec<char>>, mut carts: Vec<Cart>) {
    let mut first_crash_handled = false;
    let mut i = 0;
    while carts.len() > 1 {
        {
            let car = &mut carts[i];
            car.tick(&road);
        }

        // check
        let mut offset_i = 1;
        for j in 0..carts.len() {
            if i != j && carts[i].x == carts[j].x && carts[i].y == carts[j].y {
                if !first_crash_handled {
                    println!("result of q01 is {} {}", carts[i].x, carts[i].y);
                    first_crash_handled = true;
                }
                // remove crashed carts
                if i > j {
                    carts.remove(i);
                    carts.remove(j);
                    offset_i = -1;
                } else {
                    carts.remove(j);
                    carts.remove(i);
                    offset_i = 0;
                }
                break;
            }
            offset_i = 1;
        }

        i = (i as i32 + offset_i) as usize % carts.len();
    }
    println!("result of q02 is {} {}", carts[0].x, carts[0].y);
}

fn main() {
    let path = format!("./input/{}", "day13.txt");

    let vec: Vec<String> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut road = parse_all(vec);
    let carts = gen_carts(&mut road);
    solution(&road, carts);
}
