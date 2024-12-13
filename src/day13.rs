#![allow(dead_code)]
use crate::utils::*;
use ahash::{HashMap, HashMapExt};

/// AoC placement:
/// Time 00:12:50 rank 957
/// Result: 37901
pub fn part1(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let lines_nums = get_lines_nums(&whole);
    let grid = get_grid(whole);
    // let grid = get_grid_b(whole);
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    for n in nums.chunks_exact(6) {
        let a_x = n[0].num;
        let a_y = n[1].num;
        let b_x = n[2].num;
        let b_y = n[3].num;
        let t_x = n[4].num;
        let t_y = n[5].num;
        let mut max_a = u64::MAX;
        let mut max_b = u64::MAX;
        let mut min_cost = u64::MAX;
        for a_i in 0..100 {
            for b_i in 0..100 {
                let res_x = a_i * a_x + b_i * b_x;
                let res_y = a_i * a_y + b_i * b_y;
                if res_x == t_x && res_y == t_y {
                    let cost = a_i + b_i;
                    if cost < min_cost {
                        min_cost = cost;
                        max_a = a_i;
                        max_b = b_i;
                    }
                }
            }
        }
        if min_cost != u64::MAX {
            sum += max_a * 3 + max_b;
        }

    }
    sum as i64
}
/// AoC placement:
/// Time 00:40:02 rank 1612
/// Result: 77407675412647
pub fn part2(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let lines_nums = get_lines_nums(&whole);
    let grid = get_grid(whole);
    // let grid = get_grid_b(whole);
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    for n in nums.chunks_exact(6) {
        let a_x = n[0].num as i64;
        let a_y = n[1].num as i64;
        let b_x = n[2].num as i64;
        let b_y = n[3].num as i64;
        let t_x = n[4].num as i64 + 10000000000000;
        let t_y = n[5].num as i64 + 10000000000000;
        let mut max_a = u64::MAX;
        let mut max_b = u64::MAX;
        let mut min_cost = u64::MAX;
        
        let a_x_m = t_x / a_x;
        let b_x_m = t_x / b_x;
        
        let a_i = (t_x * b_y - t_y * b_x) / (a_x * b_y - b_x * a_y);
        let b_i = (t_y * a_x - t_x * a_y) / (a_x * b_y - b_x * a_y);
        if a_i * a_x + b_i * b_x == t_x && a_i * a_y + b_i * b_y == t_y {
            sum += a_i * 3 + b_i;
        } else {
        }

    }
    sum as i64
}
pub fn part2_no(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let lines_nums = get_lines_nums(&whole);
    let grid = get_grid(whole);
    // let grid = get_grid_b(whole);
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    for n in nums.chunks_exact(6) {
        let a_x = n[0].num;
        let a_y = n[1].num;
        let b_x = n[2].num;
        let b_y = n[3].num;
        let t_x = n[4].num + 10000000000000;
        let t_y = n[5].num + 10000000000000;
        let mut max_a = u64::MAX;
        let mut max_b = u64::MAX;
        let mut min_cost = u64::MAX;
        
        let a_x_m = t_x / a_x;
        let b_x_m = t_x / b_x;
        if a_x_m > b_x_m {
            for a_i in 0..=a_x_m {
                let added_x = a_i * a_x;
                let diff = t_x - added_x;
                let b_i = diff / b_x;
                let added_x_2 = b_i * b_x;
                if added_x + added_x_2 == t_x {
                    let gotten_y = a_i * a_y + b_i * b_y;
                    if gotten_y == t_y {
                        let cost = a_i + b_i;
                        if cost < min_cost {
                            min_cost = cost;
                            max_a = a_i;
                            max_b = b_i;
                        }
                    }
                }
            }
        } else {
            for b_i in 0..=b_x_m {
                let added_x = b_i * b_x;
                let diff = t_x - added_x;
                let a_i = diff / a_x;
                let added_x_2 = a_i * a_x;
                if added_x + added_x_2 == t_x {
                    let gotten_y = b_i * a_y + b_i * b_y;
                    if gotten_y == t_y {
                        let cost = b_i + b_i;
                        if cost < min_cost {
                            min_cost = cost;
                            max_a = b_i;
                            max_b = b_i;
                        }
                    }
                }
            }
        }

        if min_cost != u64::MAX {
            sum += max_a * 3 + max_b;
            println!("ggwp");
        } else {
            println!("aw");
        }

    }
    sum as i64
}