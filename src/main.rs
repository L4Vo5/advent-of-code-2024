#![allow(dead_code)]
use std::time::Instant;
use utils::*;
mod utils;
mod template;
mod day1;
mod day2;
mod day3;


fn main() {
    let day = 3;
    let part = 2;

    let contents = read_file(&day.to_string());
    let start = Instant::now();
    // for i in 0..10000 {
        let result = match (day, part) {
            (1, 1) => day1::part1(&contents),
            (1, 2) => day1::part2(&contents),
            (2, 1) => day2::part1(&contents),
            (2, 2) => day2::part2(&contents),
            (3, 1) => day3::part1(&contents),
            (3, 2) => day3::part2(&contents),
            (_, _) => panic!(),
        };
        // std::hint::black_box(result);
        println!("{result}");
    // }
    let end = Instant::now();
    let duration = end - start;

    println!("took {duration:?}");
}

