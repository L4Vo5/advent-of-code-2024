#![allow(dead_code)]
use crate::utils::*;
use ahash::{HashMap, HashMapExt};
use memchr::memmem;


pub fn part1(whole: &str) -> i64 {
    let whole = whole.as_bytes();
    // ah fuck
    let mut sum = 0;
    const ZERO: u8 = b'0';
    const COMMA: u8 = b',';
    const COMMA_M: u8 = COMMA.wrapping_sub(ZERO);
    const PAREN: u8 = b')';
    const PAREN_M: u8 = PAREN.wrapping_sub(ZERO);

    let mut data: [u8; 8] = [0; 8];
    for start in memmem::find_iter(whole, "mul(") {
        // whole = &whole[start+4..];
        let bytes = &whole[start+4..];
        // I'm just gonna assume this edge case doesn't happen...
        // if bytes.len() < 8 {
        //     return sum;
        // } 
        for i in 0..data.len() {
            data[i] = unsafe {bytes.get_unchecked(i).wrapping_sub(ZERO)};
        }
        let mut nums = [0i64, 0];
        let mut stage = 0;
        for &d in data.iter() {
            if d >= 10 {
                if d == COMMA_M && stage == 0 {
                    stage = 1;
                } else if d == PAREN_M && stage == 1 {
                    stage = 2;
                    break;
                } else {
                    break;
                }
            } else {
                if stage == 0 {
                    nums[0] = nums[0] * 10 + d as i64;
                } else {
                    nums[1] = nums[1] * 10 + d as i64;
                }
            }
        }
        if stage == 2 {
            sum += nums[0] * nums[1];
        } 
    }

    sum as i64
}

/// AoC placement:
/// Time 00:07:56 rank 3069
/// Result: 164730528
pub fn part1_original(mut whole: &str) -> i64 {
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