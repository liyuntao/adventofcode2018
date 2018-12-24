use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Unit {
    geo_idx: usize,
    ero_lv: usize,
    r_type: usize,
}

fn gen_grid(tar_x: usize, tar_y: usize, depth: usize) -> Vec<Vec<Unit>> {
    let mut grid = vec![
        vec![
            Unit {
                geo_idx: 0,
                ero_lv: 0,
                r_type: 0
            };
            tar_x + 1
        ];
        tar_y + 1
    ];
    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            let geo_idx = if (i == 0 && j == 0) || (i == tar_x && j == tar_y) {
                0
            } else if j == 0 {
                i * 16807
            } else if i == 0 {
                j * 48271
            } else {
                grid[j - 1][i].ero_lv * grid[j][i - 1].ero_lv
            };
            let ero_lv = (geo_idx + depth) % 20183;
            let r_type = ero_lv % 3;
            grid[j][i] = Unit {
                geo_idx,
                ero_lv,
                r_type,
            };
        }
    }
    grid
}

fn draw(grid: &Vec<Vec<Unit>>) {
    for j in 0..grid.len() {
        print!("{:03}  ", j);
        for i in 0..grid[0].len() {
            let c = match grid[j][i].r_type {
                0 => '.',
                1 => '=',
                _ => '|',
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn count(grid: &Vec<Vec<Unit>>) -> usize {
    let mut counter = 0;
    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            counter += grid[j][i].r_type;
        }
    }
    counter
}

fn q2(grid: &Vec<Vec<Unit>>, tar_x: i32, tar_y: i32) {
    // type: rocky=0,wet=1,narrow=2
    // tools:  neither=0,torch=1,gear=2
    // relation: type == cannot_used_tools
    let mut queue: VecDeque<(i32, i32, usize, usize) /* x,y,tool,minutes */> = VecDeque::new();
    let mut map: HashMap<
        (i32, i32, usize), /* x,y,tool */
        usize,             /* cur_fewest_minutes */
    > = HashMap::new();
    queue.push_back((0, 0, 1, 0));
    queue.push_back((0, 0, 2, 7));

    while !queue.is_empty() {
        let (i, j, tool, minutes) = queue.pop_front().unwrap();

        if let Some(&cur_fewest) = map.get(&(i, j, tool)) {
            if cur_fewest <= minutes {
                continue;
            }
        }

        map.insert((i, j, tool), minutes);

        // add self with tools changed
        for changed_tool in 0..3 {
            if changed_tool != tool && changed_tool != grid[j as usize][i as usize].r_type {
                queue.push_back((i, j, changed_tool, minutes + 7));
            }
        }

        // add surround without tools changed
        for t in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let new_i = i + t.0;
            let new_j = j + t.1;
            if new_i < 0 || new_j < 0
                || new_j as usize >= grid.len() || new_i as usize >= grid[0].len() {
                continue;
            }
            if grid[new_j as usize][new_i as usize].r_type == tool {
                continue;
            }
            queue.push_back((new_i, new_j, tool, minutes + 1));
        }
    }
    println!("result of q02 is {}", map.get(&(tar_x, tar_y, 1)).unwrap());
}

fn main() {
    // test
    assert_eq!(114, count(&gen_grid(10, 10, 510)));

    // q1
    let grid = gen_grid(14, 760, 7863);
//    draw(&grid);
    println!("result of q01 is {}", count(&grid));

    // q2
    let grid = gen_grid(14 + 50, 760 + 50, 7863);
    q2(&grid, 14, 760);
}
