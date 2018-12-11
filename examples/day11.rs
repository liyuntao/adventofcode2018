const M_SIZE: usize = 300;

fn get_power_level(x: usize, y: usize, sn: usize) -> i32 {
    let grid = gen_grid(sn);
    grid[y - 1][x - 1]
}

fn find_largest(sn: usize) -> (usize, usize) {
    let grid = gen_grid(sn);
    let mut max: i32 = -100;
    let mut x = 0;
    let mut y = 0;
    for i in 0..300 - 3 {
        for j in 0..300 - 3 {
            let tmp = grid[j][i]
                + grid[j][i + 1]
                + grid[j][i + 2]
                + grid[j + 1][i]
                + grid[j + 1][i + 1]
                + grid[j + 1][i + 2]
                + grid[j + 2][i]
                + grid[j + 2][i + 1]
                + grid[j + 2][i + 2];
            if tmp > max {
                max = tmp;
                x = i + 1;
                y = j + 1;
            }
        }
    }
    (x, y)
}

fn gen_grid(sn: usize) -> [[i32; M_SIZE]; M_SIZE] {
    let mut grid = [[0; M_SIZE]; M_SIZE];
    for x in 0..300 {
        for y in 0..300 {
            let rack_id = (x + 1) + 10;
            let tmp = (rack_id * (y + 1) + sn) * rack_id;
            let res = if tmp < 100 { 0 } else { (tmp % 1000) / 100 };
            grid[y][x] = res as i32 - 5;
        }
    }
    grid
}

fn main() {
    assert_eq!(-5, get_power_level(122, 79, 57));
    assert_eq!(0, get_power_level(217, 196, 39));

    let q1 = find_largest(4172);
    println!("result of q01 is {:?}", q1);
}
