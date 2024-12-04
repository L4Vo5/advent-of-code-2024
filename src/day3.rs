#![allow(dead_code)]
use crate::utils::*;
use ahash::{HashMap, HashMapExt};
use memchr::memmem;


// takes roughly 118ms for 10k iterations
pub fn part1(whole: &str) -> i64 {
    let whole = whole.as_bytes();
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

// intended result: 70478672
// takes roughly 66ms for 10k iterations
pub fn part2(whole: &str) -> i64 {
    let mut whole = whole.as_bytes();
    let mut sum = 0;
    const ZERO: u8 = b'0';
    const COMMA: u8 = b',';
    const COMMA_M: u8 = COMMA.wrapping_sub(ZERO);
    const PAREN: u8 = b')';
    const PAREN_M: u8 = PAREN.wrapping_sub(ZERO);

    let finder_mul = memmem::Finder::new("mul(");
    let finder_do = memmem::Finder::new("do()");
    let finder_dont = memmem::Finder::new("don't()");

    let mut data: [u8; 8] = [0; 8];
    let Some(mut next_mul) = finder_mul.find(whole) else {return sum};
    loop {
        let mut next_dont = finder_dont.find(whole).unwrap_or(usize::MAX);
        while next_mul < next_dont {
            // explore this mul
            whole = &whole[next_mul + 4 ..];
            next_dont = next_dont.wrapping_sub(next_mul + 4);
            if let Some(n) = finder_mul.find(whole) {
                next_mul = n;
            } else {
                return sum;
            }
            for i in 0..data.len() {
                data[i] = unsafe {whole.get_unchecked(i).wrapping_sub(ZERO)};
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
        // so we've reached a don't. advance until a do, or return.
        whole = &whole[next_dont + "don't()".len()..];
        next_mul = next_mul.wrapping_sub(next_dont + "don't()".len());
        if let Some(next_do) = finder_do.find(whole) {
            whole = &whole[next_do + "do()".len()..];
            next_mul = next_mul.wrapping_sub(next_do + "don't()".len());
            if next_mul > whole.len() {
                // underflowed, so, find new one
                if let Some(n) = finder_mul.find(whole) {
                    next_mul = n;
                } else {
                    return sum;
                }
            }
        } else {
            return sum;
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
pub fn part2_original(mut whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
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