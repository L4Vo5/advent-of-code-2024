#![allow(dead_code)]
use crate::utils::*;
use ahash::{HashMap, HashMapExt};

/// AoC placement:
/// Time 00:07:56 rank 3069
/// Result: 164730528
pub fn part1(mut whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    // ah fuck
    let mut sum = 0;

    let mut i = 0;
    while let Some(start) = whole.find("mul(") {
        whole = &whole[start+4..];
        let comma = whole.find(",");
        let end = whole.find(")");
        if comma.is_none() {
            break
        }
        if end.is_none() {
            break
        }
        if comma.unwrap()+1 >= end.unwrap() {
            continue;
        }
        let num1_s = &whole[0..comma.unwrap()];
        let num2_s = &whole[comma.unwrap()+1..end.unwrap()];
        let num1 = i32::from_str_radix(num1_s, 10);
        let num2 = i32::from_str_radix(num2_s, 10);
        if num1.is_ok() && num2.is_ok() {
            sum += num1.unwrap() * num2.unwrap();
        }

    }

    sum as i64
}
/// AoC placement:
/// Time 00:11:44 rank 1457
/// Result: 70478672
pub fn part2(mut whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    // ah fuck
    let mut sum = 0;

    let mut i = 0;
    let mut enabled = true;
    while let Some(start) = whole.find("mul(") {
        if let Some(next_dont) = whole.find("don't()") {
            if next_dont < start {
                enabled = false;
                if let Some(next_do) = whole.find("do()") {
                    whole = &whole[next_do+1..];
                    enabled = true;
                } else {
                    break;
                }
                continue;
            }
        }
        whole = &whole[start+4..];
        let comma = whole.find(",");
        let end = whole.find(")");
        if comma.is_none() {
            break
        }
        if end.is_none() {
            break
        }
        if comma.unwrap()+1 >= end.unwrap() {
            continue;
        }
        let num1_s = &whole[0..comma.unwrap()];
        let num2_s = &whole[comma.unwrap()+1..end.unwrap()];
        let num1 = i32::from_str_radix(num1_s, 10);
        let num2 = i32::from_str_radix(num2_s, 10);
        if num1.is_ok() && num2.is_ok() {
            sum += num1.unwrap() * num2.unwrap();
        }

    }

    sum as i64
}