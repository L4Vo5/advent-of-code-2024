use core::num;
use std::{fs, collections::{HashSet, HashMap, VecDeque}};
use std::time::{Instant, Duration};
use utils::*;
mod utils;
mod day1;


fn main() {
    let day = 1;
    let part = 2;

    let contents = read_file(&day.to_string());
    let start = Instant::now();
    for i in 0..10000 {
        let result = match (day, part) {
            (1, 1) => day1::part1(&contents),
            (1, 2) => day1::part2(&contents),
            (_, _) => panic!(),
        };
        std::hint::black_box(result);
        // println!("{result}");
    }
    let end = Instant::now();
    let duration = end - start;

    println!("took {duration:?}");
}

