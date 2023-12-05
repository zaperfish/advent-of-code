use std::cmp::*;
use std::collections::HashSet;
use zf_cc_utils::*;

fn main() {
    let grid = include_str!("../../input.txt").to_2d_chars();

    let mut sum = 0;
    for (ri, row) in grid.iter().enumerate() {
        for (ci, ch) in row.iter().enumerate() {
            if ch != &'*' {
                continue;
            }
            let mut set = HashSet::new();
            for x in ci.saturating_sub(1)..=min(ci + 1, row.len() - 1) {
                for y in ri.saturating_sub(1)..=min(ri + 1, grid.len() - 1) {
                    if !grid[y][x].is_digit(10) {
                        continue;
                    }
                    let mut xi = x as i32;
                    while xi >= 0 && grid[y][xi as usize].is_digit(10) {
                        xi -= 1;
                    }
                    set.insert(((xi + 1) as usize, y));
                }
            }
            if set.len() == 2 {
                let mut prod = 1;
                for (x, y) in set {
                    prod *= to_uints(grid[y].get(x..).unwrap())[0];
                }
                sum += prod;
            }
        }
    }

    println!("{}", sum);
}
