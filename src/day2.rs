use crate::utils::*;
use ahash::{HashMap, HashMapExt};

/// AoC placement:
/// Time 00:07:26 rank 1410
pub fn part1(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    // let nums = get_numbers(&whole);
    let mut sum = 0;
    for line in lines {
        let nums = get_numbers(&line);
        let mut prev = nums[0].num;
        let increasing = nums[1].num > nums[0].num;
        let mut ok = true;
        for num in nums[1..].iter() {
            let n = num.num;
            if increasing {
                if (n - prev == 0) || ( n - prev > 3) {
                    ok = false
                }
            } else {
                if (prev - n == 0) || (prev - n > 3) {
                    ok = false;
                }
            }
            prev = n;
        }
        if ok {
            sum += 1;
        }
    }
    sum
}


/// AoC placement:
/// Time 00:14:04 rank 1578
pub fn part2(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    // let nums = get_numbers(&whole);
    let mut sum = 0;
    for line in lines {
        let mut ok_final = false;
        let nums = get_numbers(&line);
        for i in 0..nums.len() {
            let mut mutated_line = nums.clone();
            mutated_line.remove(i);
            let mut prev = mutated_line[0].num;
            let increasing = mutated_line[1].num > mutated_line[0].num;
            let mut ok = true;
            for num in mutated_line[1..].iter() {
                let n = num.num;
                if increasing {
                    if (n - prev == 0) || ( n - prev > 3) {
                        ok = false
                    }
                } else {
                    if (prev - n == 0) || (prev - n > 3) {
                        ok = false;
                    }
                }
                prev = n;
            }
            if ok {
                ok_final = true;
            }
        }

        let mut prev = nums[0].num;
        let increasing = nums[1].num > nums[0].num;
        let mut ok = true;
        for num in nums[1..].iter() {
            let n = num.num;
            if increasing {
                if (n - prev == 0) || ( n - prev > 3) {
                    ok = false
                }
            } else {
                if (prev - n == 0) || (prev - n > 3) {
                    ok = false;
                }
            }
            prev = n;
        }
        if ok || ok_final {
            sum += 1;
        }
    }
    sum
}