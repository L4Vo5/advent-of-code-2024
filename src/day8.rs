#![allow(dead_code)]
use std::collections::HashSet;

use crate::utils::*;
use ahash::{HashMap, HashMapExt};

/// AoC placement:
/// Time 00:17:34 rank 2033
/// Result: 254
pub fn part1(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let grid = get_grid(whole);
    let mut sum = 0;
    let width = grid[0].len();
    let height = grid.len();

    for y in 0..height {
        for x in 0..width {
            let mut is_antinode = false;
            for yy in 0..height {
                if is_antinode {break}
                for xx in 0..width {
                    if is_antinode {break}
                    let thing = grid[yy][xx];
                    if thing == '.' {
                        continue;
                    }
                    let distance_x = xx.wrapping_sub(x);
                    let double_x = x.wrapping_add(distance_x.wrapping_mul(2));
                    if double_x >= width {continue}
                    let distance_y = yy.wrapping_sub(y);
                    let double_y = y.wrapping_add(distance_y.wrapping_mul(2));
                    if double_y >= height {continue}
                    if distance_x == 0 && distance_y == 0 {continue}
                    if double_x == x && double_y == y {
                        println!("Ouch!");
                        continue}
                    if double_x == xx && double_y == yy {
                        println!("Ouch!");
                        continue}
                    let thing2 = grid[double_y][double_x];
                    if thing == thing2 {
                        is_antinode = true;
                        break;
                    }
                }
                if is_antinode {break}
            }
            if is_antinode {
                sum += 1;
            }
        }
    }

    

    sum as i64
}
/// AoC placement:
/// Time 00:29:53 rank 2592
/// Result: 951
pub fn part2(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let grid = get_grid(whole);
    let mut sum = 0;
    let width = grid[0].len();
    let height = grid.len();

    let mut unique_antinodes = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            let antenna = grid[y][x];
            if antenna == '.' {
                continue;
            }
            for yy in 0..height {
                for xx in 0..width {
                    let antenna2 = grid[yy][xx];
                    if antenna2 != antenna {
                        continue;
                    }
                    let distance_x = xx.wrapping_sub(x);
                    let distance_y = yy.wrapping_sub(y);
                    if distance_x == 0 && distance_y == 0 {continue}
                    for i in 0.. {
                        let new_x = x.wrapping_add(distance_x.wrapping_mul(i));
                        let new_y = y.wrapping_add(distance_y.wrapping_mul(i));
                        if new_x >= width {break}
                        if new_y >= height {break}
                        unique_antinodes.insert((new_x, new_y));
                    }
                }
            }
        }
    }
    sum = unique_antinodes.len();
    println!("{unique_antinodes:?}, {width}, {height}");
    

    sum as i64
}