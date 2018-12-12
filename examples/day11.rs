const M_SIZE: usize = 301;

fn get_power_level(x: usize, y: usize, sn: usize) -> i32 {
    let grid = gen_grid(sn);
    grid[y][x]
}

fn find_largest(grid: &[[i32; M_SIZE]; M_SIZE], size: usize) -> (usize, usize, i32) {
    let mut max: i32 = -100;
    let mut x = 0;
    let mut y = 0;
    for i in 1..301 - size {
        for j in 1..301 - size {
            let mut sum_area = grid[j + size - 1][i + size - 1]
                - grid[j + size - 1][i - 1]
                - grid[j - 1][i + size - 1]
                + grid[j - 1][i - 1];
            if sum_area > max {
                max = sum_area;
                x = i;
                y = j;
            }
        }
    }
    (x, y, max)
}

fn gen_grid(sn: usize) -> [[i32; M_SIZE]; M_SIZE] {
    let mut grid = [[0; M_SIZE]; M_SIZE];
    for x in 1..M_SIZE {
        for y in 1..M_SIZE {
            let rack_id = x + 10;
            let tmp = (rack_id * y + sn) * rack_id;
            let res = if tmp < 100 { 0 } else { (tmp % 1000) / 100 };
            grid[y][x] = res as i32 - 5;
        }
    }
    grid
}

fn to_partial_sum(input: &mut [[i32; M_SIZE]; M_SIZE]) -> &[[i32; M_SIZE]; M_SIZE] {
    for y in 1..M_SIZE {
        for x in 1..M_SIZE {
            input[y][x] = input[y][x] + input[y - 1][x] + input[y][x - 1] - input[y - 1][x - 1];
        }
    }
    input
}

// for debug
fn print_grid(grid: &[[i32; M_SIZE]; M_SIZE]) {
    for y in 1..30 {
        for x in 1..30 {
            print!("{} ", grid[y][x]);
        }
        println!();
    }
}

fn main() {
    assert_eq!(-5, get_power_level(122, 79, 57));
    assert_eq!(0, get_power_level(217, 196, 39));

    let mut grid = gen_grid(4172);
    let grid = to_partial_sum(&mut grid);
    let q1 = find_largest(grid, 3);
    println!("result of q01 is {:?}", q1);

    let q2 = (1..300)
        .map(|size| (find_largest(grid, size), size))
        .max_by(|&x, &y| (x.0).2.cmp(&(y.0).2))
        .unwrap();
    println!("result of q02 is {:?}", q2);
}
