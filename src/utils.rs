#![allow(dead_code)]
use std::fs;


pub fn read_file(filename: &str) -> String {
    let path = format!("inputs/{}.txt", filename);
    fs::read_to_string(path).unwrap()
}

pub fn get_lines(content: &str) -> Vec<String> {
    content.split("\n").map(|s| s.to_string()).collect()
}

pub fn get_grid(content: &str) -> Vec<Vec<char>> {
    content.split("\n").map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
}

pub fn split_collect<'a>(s: &'a str, c: &str) -> Vec<&'a str> {
    s.split(c).collect()
}

pub fn get_numbers(s: &str) -> Vec<NumberPos> {
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
pub struct NumberPos {
    pub num: u64,
    pub s_num: i64,
    pub sign: i64, // kind of a detail of how parsing is implemented oops.
    pub i: usize, // i of the first NUMBER. if there's a minus sign before, that's in position i-1. same applies to real_i and x.
    pub real_i: usize, // taking \n into account
    pub length: usize, // length of the number in characters (-0004 is length 5)
    pub x: usize,
    pub y: usize,
}