use core::num;
use std::{fs, collections::{HashSet, HashMap, VecDeque}};
use utils::*;
mod utils;
mod day1;


fn main() {
    let day = 1;
    let part = 1;

    let result = match (day, part) {
        (1, 1) => day1::part1(&read_file("1")),
        (1, 2) => day1::part2(&read_file("1")),
        (_, _) => panic!(),
    };

    println!("{result}");
}

