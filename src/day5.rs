#![allow(dead_code)]
use crate::utils::*;
use ahash::{HashMap, HashMapExt};

/// AoC placement:
/// Time 00:09:48 rank 1270
/// Result: 5166
pub fn part1(whole: &str) -> i64 {
    let mut sum = 0;
    let spl = whole.split("\n\n").collect::<Vec<&str>>();
    let ordering_rules = spl[0];
    let updates = spl[1];
    let ordering_rules = get_lines(ordering_rules);
    let updates = get_lines(updates);
    let mut actual_rules = vec![];
    for rule in ordering_rules {
        let nums = get_numbers(&rule);
        actual_rules.push(
            (nums[0].num, nums[1].num)
        );
    }
    
    for update in updates {
        println!("{update}");
        let nums = get_numbers(&update).iter().map(|n| n.num).collect::<Vec<u64>>();
        let middle = nums[nums.len() / 2];
        let mut ok = true;
        for (i, &num) in nums.iter().enumerate() {
            for rule in actual_rules.iter() {
                if rule.0 == num {
                    for &num2 in nums[..i].iter() {
                        if rule.1 == num2 {
                            println!("broken at {num} beacuse {num2} (rule {rule:?}");
                            ok = false;
                            break;
                        }
                    }
                }
            }
        }
        if ok {
            sum += middle;
        }
    }

    sum as i64
}
/// AoC placement:
/// Time 00:15:09 rank 912
/// Result: 4679
pub fn part2(whole: &str) -> i64 {
    let mut sum = 0;
    let spl = whole.split("\n\n").collect::<Vec<&str>>();
    let ordering_rules = spl[0];
    let updates = spl[1];
    let ordering_rules = get_lines(ordering_rules);
    let updates = get_lines(updates);
    let mut actual_rules = vec![];
    for rule in ordering_rules {
        let nums = get_numbers(&rule);
        actual_rules.push(
            (nums[0].num, nums[1].num)
        );
    }
    
    for update in updates {
        println!("{update}");
        let nums = get_numbers(&update).iter().map(|n| n.num).collect::<Vec<u64>>();
        let middle = nums[nums.len() / 2];
        let mut ok = true;
        for (i, &num) in nums.iter().enumerate() {
            for rule in actual_rules.iter() {
                if rule.0 == num {
                    for &num2 in nums[..i].iter() {
                        if rule.1 == num2 {
                            println!("broken at {num} beacuse {num2} (rule {rule:?}");
                            ok = false;
                            break;
                        }
                    }
                }
            }
        }
        if !ok {
            // fix it
            let mut new_nums = nums.clone();
            use std::cmp::Ordering::*;
            new_nums.sort_by(|&a, &b| {
                let mut ret = Equal;
                for rule in actual_rules.iter() {
                    if rule.0 == a && rule.1 == b {
                        // a < b
                        ret = Less;
                        break;
                    }
                    if rule.0 == b && rule.1 == a {
                        ret = Greater;
                        break;
                    }
                }
                ret
            });

            let middle = new_nums[new_nums.len() / 2];
            sum += middle;
        }
    }

    sum as i64
}