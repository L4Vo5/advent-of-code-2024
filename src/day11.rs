#![allow(dead_code)]
use std::collections::HashSet;

use crate::utils::*;
use ahash::{HashMap, HashMapExt};

/// AoC placement:
/// Time 00:09:04 rank 1927
/// Result: 182081
pub fn part1(whole: &str) -> i64 {
    fn num_from_digits(digits: &[u64]) -> u64 {
        let mut n = 0;
        for &d in digits {
            n *= 10;
            n += d;
        }
        n
    }
    fn digits_from_num(mut n: u64) -> Vec<u64> {
        let mut d = vec![];
        while n != 0 {
            d.push(n % 10);
            n /= 10;
        }
        d.iter().rev().copied().collect()
    }
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let grid = get_grid(whole);
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    let mut stones_prev = nums.iter().map(|n| n.num).collect::<Vec<u64>>();
    for i in 0..25 {
        let mut stones_now = vec![];
        for &s in stones_prev.iter() {
            if s == 0 {
                stones_now.push(1);
                continue
            }
            let mut digits = digits_from_num(s);
            if digits.len() % 2 == 1 {
                stones_now.push(s * 2024);
            } else {
                stones_now.push(num_from_digits(&digits[0..digits.len()/2]));
                stones_now.push(num_from_digits(&digits[digits.len()/2..]));
            }
        }
        stones_prev = stones_now;
        println!("i = {i}, partial = {}", stones_prev.len());
    }
    sum = stones_prev.len();
    let unique = HashSet::<u64>::from_iter(stones_prev.iter().copied());
    println!("unique = {}", unique.len());
    sum as i64
}
/// AoC placement:
/// Time 00:44:49 rank 3662
/// Result: 216318908621637
pub fn part2(whole: &str) -> i64 {
    fn num_from_digits(digits: &[u64]) -> u64 {
        let mut n = 0;
        for &d in digits {
            n *= 10;
            n += d;
        }
        n
    }
    fn digits_from_num(mut n: u64) -> Vec<u64> {
        let mut d = vec![];
        while n != 0 {
            d.push(n % 10);
            n /= 10;
        }
        d.iter().rev().copied().collect()
    }
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let grid = get_grid(whole);
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    let mut stones_prev = nums.iter().map(|n| n.num).collect::<Vec<u64>>();
    let mut stones_prev = HashMap::<u64, usize>::from_iter(stones_prev.iter().map(|s| (*s, 1)));
    for i in 0..750 {
        let mut stones_now = HashMap::new();
        for (&s, &count) in stones_prev.iter() {
            if s == 0 {
                stones_now.entry(1).and_modify(|n| {*n += count;}).or_insert(count);
                continue
            }
            let mut digits = digits_from_num(s);
            if digits.len() % 2 == 1 {
                stones_now.entry(s * 2024).and_modify(|n| {*n += count;}).or_insert(count);
            } else {
                let n1 = num_from_digits(&digits[0..digits.len()/2]);
                let n2 = num_from_digits(&digits[digits.len()/2..]);
                stones_now.entry(n1).and_modify(|n| {*n += count;}).or_insert(count);
                stones_now.entry(n2).and_modify(|n| {*n += count;}).or_insert(count);
            }
        }
        stones_prev = stones_now;
        // sum = 0;
        // for (&s, &count) in stones_prev.iter() {
        //     sum += count;
        // }
        // println!("i = {i}, partial = {sum} unique = {}", stones_prev.len());
    }
    sum = 0;
    for (&s, &count) in stones_prev.iter() {
        sum += count;
    }
    // println!("{stones_prev:?}");
    // NOT 80375505489347 SOMEHOW???
    // 80375505489347
    
    sum as i64
}
pub fn part2_no(whole: &str) -> i64 {
    fn num_from_digits(digits: &[u64]) -> u64 {
        let mut n = 0;
        for &d in digits {
            n *= 10;
            n += d;
        }
        n
    }
    fn digits_from_num(mut n: u64) -> Vec<u64> {
        let mut d = vec![];
        while n != 0 {
            d.push(n % 10);
            n /= 10;
        }
        d.iter().rev().copied().collect()
    }
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let grid = get_grid(whole);
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    let mut stones_prev = nums.iter().map(|n| n.num).collect::<Vec<u64>>();
    fn eval_stone(n: u64, remaining_steps: usize) -> usize {
        if remaining_steps == 0 {
            1
        } else {
            if n == 0 {
                return eval_stone(1, remaining_steps - 1)
            }
            let digits = digits_from_num(n);
            if digits.len() % 2 == 1 {
                eval_stone(n * 2024, remaining_steps - 1)
            } else {
                eval_stone(num_from_digits(&digits[0..digits.len()/2]), remaining_steps - 1) +
                eval_stone(num_from_digits(&digits[digits.len()/2..]), remaining_steps - 1)
            }
        }
    }
    sum = stones_prev.iter().map(|s| eval_stone(*s, 75)).sum();

    sum as i64
}