#![allow(dead_code)]
use crate::utils::*;
use ahash::{HashMap, HashMapExt};

/// AoC placement:
/// Time 00:06:29 rank 290
/// Result: 169122112716571
pub fn part2(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let grid = get_grid(whole);
    let mut sum = 0;

    fn try_values(values: &[u64], current: u64, obj: u64) -> bool {
        if values.len() == 0 {
            return current == obj;
        }
        try_values(&values[1..], current * values[0], obj)
        || try_values(&values[1..], current + values[0], obj)
        || try_values(&values[1..],
            // concatenation
            {
                let n = values[0];
                let mut digits = 0;
                let mut mul = 1;
                let mut m = n;
                while m != 0 {
                    m /= 10;
                    digits += 1;
                    mul *= 10;
                }
                current * mul + n
            }
            , obj)
    } 

    for line in lines {
        let nums = get_numbers(&line);
        let test_value = nums[0].num;
        let other_values = nums[1..].iter().map(|n| n.num).collect::<Vec<u64>>();
        if try_values(&other_values, 0, test_value) {
            sum += test_value;
        }
    }
    sum as i64
}
/// AoC placement:
/// Time 00:03:45 rank 260
/// Result: 169122112716571
pub fn part1(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let grid = get_grid(whole);
    let mut sum = 0;

    fn try_values(values: &[u64], current: u64, obj: u64) -> bool {
        if values.len() == 0 {
            return current == obj;
        }
        try_values(&values[1..], current * values[0], obj)
        || try_values(&values[1..], current + values[0], obj)
    } 

    for line in lines {
        let nums = get_numbers(&line);
        let test_value = nums[0].num;
        let other_values = nums[1..].iter().map(|n| n.num).collect::<Vec<u64>>();
        if try_values(&other_values, 0, test_value) {
            sum += test_value;
        }
    }
    sum as i64
}