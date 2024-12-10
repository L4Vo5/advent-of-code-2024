#![allow(dead_code)]
use std::collections::HashSet;

use crate::utils::*;
use ahash::{HashMap, HashMapExt};

/// AoC placement:
/// Time 00:07:18 rank 597
/// Result: 574
pub fn part1(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let grid = get_grid_b(whole);
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    fn traverse(grid: &Vec<Vec<u8>>, pos: (usize, usize), elevation: u8) -> HashSet<(usize, usize)> {
        let width = grid[0].len();
        let height = grid.len();
        let (x, y) = pos;
        if (grid[y][x] - b'0') != elevation {
            return HashSet::new();
        }
        if elevation == 9 {
            let mut hs = HashSet::new();
            hs.insert((x, y));
            return hs;
        }
        let mut hs = HashSet::new();
        if x > 0 {
            hs.extend(traverse(grid, (x-1, y), elevation+1).iter());
        }
        if y > 0 {
            hs.extend(traverse(grid, (x, y-1), elevation+1).iter());
        }
        if x < width-1 {
            hs.extend(traverse(grid, (x+1, y), elevation+1).iter());
        }
        if y < height-1 {
            hs.extend(traverse(grid, (x, y+1), elevation+1).iter());
        }
        hs
    }
    for y in 0..height {
        for x in 0..width {
            sum += traverse(&grid, (x, y), 0).len();
        }
    }

    sum as i64
}
/// AoC placement:
/// Time 00:08:04 rank 358
/// Result: 1238
/// Funny, I solved part 1 like this as a mistake. So solving part 2 was a matter of pressing ctrl+z for a bit.
pub fn part2(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let grid = get_grid_b(whole);
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    fn traverse(grid: &Vec<Vec<u8>>, pos: (usize, usize), elevation: u8) -> usize {
        let width = grid[0].len();
        let height = grid.len();
        let (x, y) = pos;
        if (grid[y][x] - b'0') != elevation {
            return 0;
        }
        if elevation == 9 {
            return 1;
        }
        let mut sum = 0;
        if x > 0 {
            sum += traverse(grid, (x-1, y), elevation+1);
        }
        if y > 0 {
            sum += traverse(grid, (x, y-1), elevation+1);
        }
        if x < width-1 {
            sum += traverse(grid, (x+1, y), elevation+1);
        }
        if y < height-1 {
            sum += traverse(grid, (x, y+1), elevation+1);
        }
        sum
    }
    for y in 0..height {
        for x in 0..width {
            sum += traverse(&grid, (x, y), 0);
        }
    }
    
    sum as i64
}

