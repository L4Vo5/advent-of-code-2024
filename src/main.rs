use core::num;
use std::{fs, collections::{HashSet, HashMap, VecDeque}};


fn read_file(filename: &str) -> Vec<String> {
    let path = format!("inputs/{}.txt", filename);
    let content = fs::read_to_string(path).unwrap();
    content.split("\n").map(|s| s.to_string()).collect()
}
fn read_file_whole(filename: &str) -> String {
    let path = format!("inputs/{}.txt", filename);
    fs::read_to_string(path).unwrap()
}

fn split_collect<'a>(s: &'a str, c: &str) -> Vec<&'a str> {
    s.split(c).collect()
}

fn get_numbers(s: &str) -> Vec<NumberPos> {
    let mut v = vec![];
    let mut last_was_num = false;
    let mut i = 0;
    let mut real_i = 0;
    for (y, line) in s.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char.is_numeric() {
                let this_num = char.to_digit(10).unwrap() as u64;
                if last_was_num {
                    let last: &mut NumberPos = v.last_mut().unwrap();
                    last.num = last.num * 10 + this_num;
                    last.s_num = last.num as i64 * last.sign;
                    last.length += 1;
                } else {
                    let mut sign = 1;
                    if i != 0 && &s[i-1..=i-1] == "-" {sign = -1;}
                    let s_num = this_num as i64 * sign;
                    v.push(NumberPos { num: this_num, s_num, i, x, y, real_i, length: if sign == 1 {1} else {2}, sign})
                }
                last_was_num = true;
            } else {
                last_was_num = false;
            }
            i += 1;
            real_i += 1;
        }
        last_was_num = false;
        real_i += 1;
    }
    v
}

#[derive(Debug, Clone, Copy)]
struct NumberPos {
    num: u64,
    s_num: i64,
    sign: i64, // kind of a detail of how parsing is implemented oops.
    i: usize, // i of the first NUMBER. if there's a minus sign before, that's in position i-1. same applies to real_i and x.
    real_i: usize, // taking \n into account
    length: usize, // length of the number in characters (-0004 is length 5)
    x: usize,
    y: usize,
}

fn main() {
    day_1_p2();
}

fn day_1_p1() {
    let lines = read_file("1");
    let whole = read_file_whole("1");
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
    println!("{sum}");

}

fn day_1_p2() {
    let lines = read_file("1");
    let whole = read_file_whole("1");
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
    println!("{sum}");

}


