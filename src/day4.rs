#![allow(dead_code)]
use crate::utils::*;
use ahash::{HashMap, HashMapExt};

/// AoC placement:
/// Time 00:06:24 rank 827
/// Result: 2547
pub fn part1(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let mut sum = 0;
    let grid = get_grid(whole);
    let height = grid.len();
    let width = grid[0].len();
    let directions = [
        (1, 1), (1, 0), (1, -1), (0, 1), (0, -1), (-1, -1), (-1, 0), (-1, 1)
    ];
    for x in 0..width {
        for y in 0..height {
            if grid[y][x] == 'X' {
                let x = x as i64;
                let y = y as i64;
                for dir in directions {
                    let xx1 = x + dir.0;
                    let yy1 = y + dir.1;
                    let xx2 = x + dir.0 * 2;
                    let yy2 = y + dir.1 * 2;
                    let xx3 = x + dir.0 * 3;
                    let yy3 = y + dir.1 * 3;
                    if xx3 < 0 || xx3 >= width as i64 || yy3 < 0 || yy3 >= height as i64 {
                        continue;
                    } else {
                        let xx1 = xx1 as usize;
                        let yy1 = yy1 as usize;
                        let xx2 = xx2 as usize;
                        let yy2 = yy2 as usize;
                        let xx3 = xx3 as usize;
                        let yy3 = yy3 as usize;
                        if grid[yy1][xx1] == 'M' && grid[yy2][xx2] == 'A' && grid[yy3][xx3] == 'S' {
                            sum += 1;
                        }
                    }
                }
            }
        }
    }

    sum as i64
}
/// AoC placement:
/// Time 00:14:05 rank 831
/// Result: 1939
pub fn part2(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let mut sum = 0;
    let grid = get_grid(whole);
    let height = grid.len();
    let width = grid[0].len();
    let directions = [
        (1, 1), (1, 0), (1, -1), (0, 1), (0, -1), (-1, -1), (-1, 0), (-1, 1)
    ];
    for x in 0..width {
        for y in 0..height {
            if grid[y][x] == 'M' {
                let x = x as i64;
                let y = y as i64;
                let positions = [
                    (x + 1, y + 1, 'A'),
                    (x + 2, y + 2, 'S'),
                    (x + 2, y + 0, 'S'),
                    (x + 0, y + 2, 'M'),
                ];
                let mut ok = true;
                for pos in positions.iter().copied() {
                    if pos.0 < 0 || pos.0 >= width as i64 || pos.1 < 0 || pos.1 >= height as i64 {
                        ok = false;
                    }
                }
                if ok {
                    for pos in positions.iter().copied() {
                        let xx = pos.0;
                        let yy = pos.1;
                        if grid[yy as usize][xx as usize] != pos.2 {
                            ok = false;
                        }
                    }
                    if ok {
                        sum += 1;
                    }
                }
                let positions = [
                    (x + 1, y + 1, 'A'),
                    (x + 2, y + 2, 'S'),
                    (x + 2, y + 0, 'M'),
                    (x + 0, y + 2, 'S'),
                ];
                let mut ok = true;
                for pos in positions.iter().copied() {
                    if pos.0 < 0 || pos.0 >= width as i64 || pos.1 < 0 || pos.1 >= height as i64 {
                        ok = false;
                    }
                }
                if ok {
                    for pos in positions.iter().copied() {
                        let xx = pos.0;
                        let yy = pos.1;
                        if grid[yy as usize][xx as usize] != pos.2 {
                            ok = false;
                        }
                    }
                    if ok {
                        sum += 1;
                    }
                }
                let positions = [
                    (x - 1, y + 1, 'A'),
                    (x - 2, y + 2, 'S'),
                    (x - 2, y + 0, 'S'),
                    (x - 0, y + 2, 'M'),
                ];
                let mut ok = true;
                for pos in positions.iter().copied() {
                    if pos.0 < 0 || pos.0 >= width as i64 || pos.1 < 0 || pos.1 >= height as i64 {
                        ok = false;
                    }
                }
                if ok {
                    for pos in positions.iter().copied() {
                        let xx = pos.0;
                        let yy = pos.1;
                        if grid[yy as usize][xx as usize] != pos.2 {
                            ok = false;
                        }
                    }
                    if ok {
                        sum += 1;
                    }
                }
                let positions = [
                    (x + 1, y - 1, 'A'),
                    (x + 2, y - 2, 'S'),
                    (x + 2, y - 0, 'M'),
                    (x + 0, y - 2, 'S'),
                ];
                let mut ok = true;
                for pos in positions.iter().copied() {
                    if pos.0 < 0 || pos.0 >= width as i64 || pos.1 < 0 || pos.1 >= height as i64 {
                        ok = false;
                    }
                }
                if ok {
                    for pos in positions.iter().copied() {
                        let xx = pos.0;
                        let yy = pos.1;
                        if grid[yy as usize][xx as usize] != pos.2 {
                            ok = false;
                        }
                    }
                    if ok {
                        sum += 1;
                    }
                }
            }
        }
    }

    sum as i64
}