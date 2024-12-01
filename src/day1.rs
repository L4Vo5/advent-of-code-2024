use crate::utils::*;

pub fn part1(whole: &str) -> i64 {
    // Take advantage of the fact that the format for each line is:
    // (S = Space)
    const FORMAT: &str = "AAAAASSSBBBBB\n";
    const ZERO: u8 = b'0';
    let whole = whole.as_bytes();
    let line_count = whole.len() / FORMAT.len();
    let mut nums_1 = vec![0; line_count];
    let mut nums_2 = vec![0; line_count];
    for (i, chunk) in whole.chunks_exact(FORMAT.len()).enumerate() {
        let n1 = 
        (chunk[4] - ZERO) as u32 +
        (chunk[3] - ZERO) as u32 * 10 +
        (chunk[2] - ZERO) as u32 * 100 +
        (chunk[1] - ZERO) as u32 * 1000 +
        (chunk[0] - ZERO) as u32 * 10000;
        let n2 = 
        (chunk[12] - ZERO) as u32 +
        (chunk[11] - ZERO) as u32 * 10 +
        (chunk[10] - ZERO) as u32 * 100 +
        (chunk[9] - ZERO) as u32 * 1000 +
        (chunk[8] - ZERO) as u32 * 10000;
        nums_1[i] = n1;
        nums_2[i] = n2;
    }
    nums_1.sort();
    nums_2.sort();
    let mut sum = 0;
    for i in 0..line_count {
        sum += nums_1[i].abs_diff(nums_2[i]);
    }
    sum as i64
}

/// AoC placement:
/// Time 00:03:10 rank 857
pub fn part1_original(whole: &str) -> i64 {
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
    sum as i64
}

/// AoC placement:
/// Time 00:04:52 rank 710
pub fn part2(whole: &str) -> i64 {

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
    sum as i64
}