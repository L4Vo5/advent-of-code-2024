#![allow(dead_code)]
use crate::utils::*;
use ahash::{HashMap, HashMapExt};

/// AoC placement:
/// Time 00:05:55 rank 104
/// Result: 6463499258318
pub fn part1(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let grid = get_grid(whole);
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    let mut actual_fs = vec![];
    let mut switch = true;
    let mut current_file = 0;
    for c in whole.bytes() {
        let n = c - b'0';
        let n = n as usize;
        for i in 0..n {
            if switch {
                actual_fs.push(current_file as isize);
            } else {
                actual_fs.push(-1);
            }
        }
        if switch {
            current_file += 1;
        }
        switch = !switch;
    }
    let mut j = actual_fs.len() - 1;
    let mut i = 0;
    while i <= j {
        if actual_fs[i] != -1 {
            sum += actual_fs[i] * i as isize;
            i += 1;
        } else {
            while actual_fs[j] == -1 {
                j -= 1;
            }
            sum += actual_fs[j] * i as isize;
            i += 1;
            j -= 1;
        }
    }

    sum as i64
}
/// AoC placement:
/// Time 00:18:07 rank 172
/// Result: 6493634986625
pub fn part2(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let grid = get_grid(whole);
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    let mut actual_fs = vec![];
    let mut switch = true;
    let mut current_file = 0;
    let mut files = vec![];
    let mut gaps = vec![];
    for c in whole.bytes() {
        let n = c - b'0';
        let n = n as usize;
        if switch {
            files.push((actual_fs.len(), n));
        } else {
            gaps.push((actual_fs.len(), n));
        }
        for i in 0..n {
            if switch {
                actual_fs.push(current_file as isize);
            } else {
                actual_fs.push(-1);
            }
        }
        if switch {
            current_file += 1;
        }
        switch = !switch;
    }

    for (file_i, file) in files.iter().enumerate().rev() {
        
        for gap in gaps.iter_mut() {
            // a good 5 minutes of my time were in realizing I needed this break condition lol
            if gap.0 >= file.0 {
                break;
            }
            if gap.1 >= file.1 {
                // erase file
                for i in file.0..file.0+file.1 {
                    assert!(actual_fs[i] == file_i as isize);
                    actual_fs[i] = -1;
                }
                // put file in start of gap
                for i in gap.0..gap.0 + file.1 {
                    assert!(actual_fs[i] == -1);
                    actual_fs[i] = file_i as isize;
                }
                // shrink gap
                gap.1 -= file.1;
                gap.0 += file.1;
                break;
            }
        }
    }

    for i in 0..actual_fs.len() {
        if actual_fs[i] != -1 {
            sum += actual_fs[i] * i as isize;
        }
    }

    sum as i64
}