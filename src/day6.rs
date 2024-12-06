#![allow(dead_code)]
use crate::utils::*;
use ahash::{HashMap, HashMapExt};
use std::collections::HashSet;

/// AoC placement:
/// Time 00:07:09 rank 588
/// Result: 5318
pub fn part1(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let grid = get_grid(whole);
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    let mut distinct_positions = HashSet::new();
    let (mut x, mut y) = {
        let mut current_position = (0, 0);
        for x in 0..width {
            for y in 0..height {
                if grid[y][x] == '^' {
                    current_position = (x, y);
                    break;
                }
            }
        }
        current_position
    };
    let directions = [
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
    ];
    let mut current_direction_i = 0;
    let mut current_direction = directions[current_direction_i];
    for i in 0..width*height*4*2 {
        distinct_positions.insert((x, y));
        x = x.wrapping_add(current_direction.0 as usize);
        y = y.wrapping_add(current_direction.1 as usize);
        if x >= width || y >= height {
            break;
        }
        if grid[y][x] == '#' {
            // obstacle
            x = x.wrapping_sub(current_direction.0 as usize);
            y = y.wrapping_sub(current_direction.1 as usize);
            current_direction_i += 1;
            current_direction = directions[current_direction_i % 4];
        }
    }


    sum = distinct_positions.len();
    sum as i64
}
/// AoC placement:
/// Time 00:12:10 rank 233
/// Result: 1831
pub fn part2(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let grid = get_grid(whole);
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;


    let mut distinct_positions_original = HashSet::new();
    let (mut x, mut y) = {
        let mut current_position = (0, 0);
        for x in 0..width {
            for y in 0..height {
                if grid[y][x] == '^' {
                    current_position = (x, y);
                    break;
                }
            }
        }
        current_position
    };
    let starting_x = x;
    let starting_y = y;
    let directions = [
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
    ];
    let mut current_direction_i = 0;
    let mut current_direction = directions[current_direction_i];
    let limit = width*height*4*2;
    for i in 0..limit {
        distinct_positions_original.insert((x, y));
        x = x.wrapping_add(current_direction.0 as usize);
        y = y.wrapping_add(current_direction.1 as usize);
        if x >= width || y >= height {
            break;
        }
        if grid[y][x] == '#' {
            // obstacle
            x = x.wrapping_sub(current_direction.0 as usize);
            y = y.wrapping_sub(current_direction.1 as usize);
            current_direction_i += 1;
            current_direction = directions[current_direction_i % 4];
        }
    }

    let mut loop_count = 0;
    let original_grid = grid;
    for (xpos, ypos) in distinct_positions_original.iter().copied() {
        let mut grid = original_grid.clone();
        grid[ypos][xpos] = '#';
        let mut x = starting_x;
        let mut y = starting_y;
        if x == xpos && y == ypos {
            continue;
        }
        let mut distinct_positions = HashSet::new();
        let mut current_direction_i = 0;
        let mut current_direction = directions[current_direction_i];
        let limit = width*height*4*2;
        for i in 0..limit {
            distinct_positions.insert((x, y));
            x = x.wrapping_add(current_direction.0 as usize);
            y = y.wrapping_add(current_direction.1 as usize);
            if x >= width || y >= height {
                break;
            }
            if grid[y][x] == '#' {
                // obstacle
                x = x.wrapping_sub(current_direction.0 as usize);
                y = y.wrapping_sub(current_direction.1 as usize);
                current_direction_i += 1;
                current_direction = directions[current_direction_i % 4];
            }
            if i == limit-1 {
                loop_count += 1;
                break;
            }
        }
    }


    sum = loop_count;
    sum as i64
}