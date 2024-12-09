#![allow(dead_code)]
use crate::utils::*;
use ahash::{HashMap, HashMapExt};

/// AoC placement:
/// Time xx:xx:xx rank xxxx
/// Result:
pub fn part1(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let grid = get_grid(whole);
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    sum as i64
}
/// AoC placement:
/// Time xx:xx:xx rank xxxx
/// Result:
pub fn part2(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let grid = get_grid(whole);
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    sum as i64
}