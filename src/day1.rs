use crate::utils::*;

/// AoC placement:
/// Time 00:03:10 rank 857
pub fn part1(whole: &str) -> String {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let mut nums_1 = vec![];
    let mut nums_2 = vec![];
    for i in 0..nums.len() {
        if i % 2 == 0 {
            nums_1.push(nums[i].num);
        } else {
            nums_2.push(nums[i].num);
        }
    }
    nums_1.sort();
    nums_2.sort();
    let mut sum = 0;
    for i in 0..nums_1.len() {

        sum += (nums_1[i] as i64 - nums_2[i] as i64).abs();
    }
    sum.to_string()
}

/// AoC placement:
/// Time 00:04:52 rank 710
pub fn part2(whole: &str) -> String {

    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let mut nums_1 = vec![];
    let mut nums_2 = vec![];
    for i in 0..nums.len() {
        if i % 2 == 0 {
            nums_1.push(nums[i].num);
        } else {
            nums_2.push(nums[i].num);
        }
    }
    nums_1.sort();
    nums_2.sort();
    let mut sum = 0;
    for i in 0..nums_1.len() {
        let n = nums_1[i];
        let mut a = 0;
        for j in 0..nums_2.len() {
            if nums_2[j] == n {
                a += 1;
            }
        }
        sum += n * a;
    }
    sum.to_string()
}