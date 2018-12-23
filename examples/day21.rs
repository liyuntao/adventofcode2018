use std::cmp::max;
use std::collections::HashSet;

fn main() {
    let mut b: u128 = 0;
    let mut e: u128 = 0;

    let mut seen = HashSet::new();
    let mut pre_e = 0;

    loop {
        b = e | 65536;
        e = 2024736; // your magical number, change here

        loop {
            e = (e + (b & 255) & 16777215) * 65899 & 16777215;
            if b < 256 {
                break;
            }
            b = b / 256;
        }

        if seen.is_empty() {
            println!("result of q01 is {}", e);
        }

        if !seen.insert(e) {
            println!("result of q02 is {}", pre_e);
            break;
        }
        pre_e = e;
    }
}
