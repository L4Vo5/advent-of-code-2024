#![allow(dead_code)]
use crate::utils::*;
use ahash::{HashMap, HashMapExt};

/// AoC placement:
/// Time 00:22:47 rank 2452
/// Result: 216772608
pub fn part1(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let mut nums = get_numbers(&whole);
    let grid = get_grid(whole);
    // let grid = get_grid_b(whole);
    let width = 101;
    let height = 103;

    let mut robots = vec![];
    while !nums.is_empty() {
        let (y_spd, x_spd, y, x) = (nums.pop().unwrap().s_num, nums.pop().unwrap().s_num, nums.pop().unwrap().s_num, nums.pop().unwrap().s_num);
        println!("{y_spd}, {x_spd}, {y}, {x}");
        robots.push(
            (x, y, x_spd, y_spd)
        );
    }
    let mut sum = 0;
    for i in 0..100 {
        for r in robots.iter_mut() {
            r.0 += r.2;
            r.1 += r.3;
            while r.0 < 0 {
                r.0 += width;
            }
            while r.0 >= width {
                r.0 -= width;
            }
            while r.1 < 0 {
                r.1 += height;
            }
            while r.1 >= height {
                r.1 -= height;
            }
        }
    }
    let mut quadrants = [
        0, 0, 0, 0
    ];
    for r in robots.iter() {
        if r.0 < width/2 && r.1 < height/2 {
            quadrants[0] += 1;
        }
        if r.0 > width/2 && r.1 < height/2 {
            quadrants[1] += 1;
        }
        if r.0 > width/2 && r.1 > height/2 {
            quadrants[2] += 1;
        }
        if r.0 < width/2 && r.1 > height/2 {
            quadrants[3] += 1;
        }
    }
    sum = 1;
    println!("{quadrants:?}");
    println!("{}", robots.len());
    let mut s = String::new();
    for y in 0..height {
        for x in 0..width {
            let mut found = 0;
            for r in robots.iter() {
                if r.0 == x && r.1 == y {
                    found += 1;
                }
            }
            if found == 0 {
                s += ".";
            } else {
                s += &format!("{found}");
            }
        }
        s += "\n"
    }
    println!("{s}");
    for q in quadrants.iter() {
        sum *= q;
    }
    sum as i64
}
/// AoC placement:
/// Time 00:40:51 rank 1428
/// Result:
pub fn part2(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let mut nums = get_numbers(&whole);
    let grid = get_grid(whole);
    // let grid = get_grid_b(whole);
    let width = 101;
    let height = 103;

    let mut robots = vec![];
    while !nums.is_empty() {
        let (y_spd, x_spd, y, x) = (nums.pop().unwrap().s_num, nums.pop().unwrap().s_num, nums.pop().unwrap().s_num, nums.pop().unwrap().s_num);
        println!("{y_spd}, {x_spd}, {y}, {x}");
        robots.push(
            (x, y, x_spd, y_spd)
        );
    }
    let mut sum = 0;
    for i in 0..100000 {
        for r in robots.iter_mut() {
            r.0 += r.2;
            r.1 += r.3;
            while r.0 < 0 {
                r.0 += width;
            }
            while r.0 >= width {
                r.0 -= width;
            }
            while r.1 < 0 {
                r.1 += height;
            }
            while r.1 >= height {
                r.1 -= height;
            }
        }
        if (i + 1 - 121) % 101 == 0 {
            println!("After {} seconds...", i+1);
            let mut s = String::new();
            for y in 0..height {
                for x in 0..width {
                    let mut found = 0;
                    for r in robots.iter() {
                        if r.0 == x && r.1 == y {
                            found += 1;
                        }
                    }
                    if found == 0 {
                        s += ".";
                    } else {
                        s += &format!("{found}");
                    }
                }
                s += "\n"
            }
            println!("{s}");
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    let mut quadrants = [
        0, 0, 0, 0
    ];
    for r in robots.iter() {
        if r.0 < width/2 && r.1 < height/2 {
            quadrants[0] += 1;
        }
        if r.0 > width/2 && r.1 < height/2 {
            quadrants[1] += 1;
        }
        if r.0 > width/2 && r.1 > height/2 {
            quadrants[2] += 1;
        }
        if r.0 < width/2 && r.1 > height/2 {
            quadrants[3] += 1;
        }
    }
    sum = 1;
    println!("{quadrants:?}");
    println!("{}", robots.len());
    for q in quadrants.iter() {
        sum *= q;
    }
    sum as i64
}