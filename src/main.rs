#![allow(dead_code)]
use std::time::Instant;
use utils::*;
mod utils;
mod template;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;


fn main() {
    let day = 9;
    // let part = 1;

    for part in 1..=2 {
        let start = Instant::now();
        let result = run(day, part);
        let end = Instant::now();
        println!("day {day} part {part} result: {result}");
        let duration = end - start;
        println!("took {duration:?}");
    }
}

fn run(day: i32, part: i32) -> i64 {
    let contents = read_file(&day.to_string());
    match (day, part) {
        (1, 1) => day1::part1(&contents),
        (1, 2) => day1::part2(&contents),
        (2, 1) => day2::part1(&contents),
        (2, 2) => day2::part2(&contents),
        (3, 1) => day3::part1(&contents),
        (3, 2) => day3::part2(&contents),
        (4, 1) => day4::part1(&contents),
        (4, 2) => day4::part2(&contents),
        (5, 1) => day5::part1(&contents),
        (5, 2) => day5::part2(&contents),
        (6, 1) => day6::part1(&contents),
        (6, 2) => day6::part2(&contents),
        (7, 1) => day7::part1(&contents),
        (7, 2) => day7::part2(&contents),
        (8, 1) => day8::part1(&contents),
        (8, 2) => day8::part2(&contents),
        (9, 1) => day9::part1(&contents),
        (9, 2) => day9::part2(&contents),
        (10, 1) => day10::part1(&contents),
        (10, 2) => day10::part2(&contents),
        (_, _) => panic!(),
    }
}
