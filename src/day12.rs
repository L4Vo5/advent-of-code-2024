#![allow(dead_code)]
use std::collections::HashSet;

use crate::utils::*;
use ahash::{HashMap, HashMapExt};

/// AoC placement:
/// Time 00:28:51 rank 3335
/// Result: 1375476
pub fn part1(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let grid = get_grid(whole);
    let grid = get_grid_b(whole);
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    let mut regions = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            let c = grid[y][x];
            regions.entry(c).or_insert(vec![]).push((x, y));
        }
    }
    // subdivide regions further
    let regions_orig = regions;
    let mut regions = HashMap::new();
    for (char, cells) in regions_orig.iter() {
        let occupied: HashSet<(usize, usize)> = HashSet::from_iter(cells.iter().copied());
        let mut pooled_cells = HashMap::new();
        let mut pools = HashMap::new();
        for (i, cell) in cells.iter().enumerate() {
            pooled_cells.insert(*cell, i);
        }
        for i in 0..cells.len() {
            pools.insert(i, vec![cells[i]]);
        }
        for (i, (x, y)) in cells.iter().enumerate() {
            let my_pool_i = *pooled_cells.get(&(*x, *y)).unwrap();
            let sides: [(i32, i32); 4] = [
                (-1, 0), (0, -1), (1, 0), (0, 1)
            ];
            for side in sides {
                let xx = x.wrapping_add(side.0 as usize);
                let yy = y.wrapping_add(side.1 as usize);
                if occupied.contains(&(xx, yy)) {
                    // move all cells from that pool into this one's.
                    let other_pool_i = *pooled_cells.get(&(xx, yy)).unwrap();
                    if other_pool_i == my_pool_i {
                        continue;
                    }
                    let other_pool = pools.get(&other_pool_i).unwrap().clone();
                    // empty the other pool
                    pools.remove(&other_pool_i);
                    
                    // set the cells from that pool, to my pool
                    for (xx, yy) in other_pool.iter().copied() {
                        *pooled_cells.get_mut(&(xx, yy)).unwrap() = my_pool_i;
                    }

                    // extend my pool
                    let my_pool = pools.get_mut(&my_pool_i).unwrap();
                    my_pool.extend(other_pool);
                }
            }
        }
        // each pool now has a region
        regions.insert(char, pools.into_values().collect::<Vec<Vec<(usize, usize)>>>());
    }

    for (&char, cell_regions) in regions.iter() {
        for cells in cell_regions {
            let char = char::from_u32(*char as u32).unwrap();
            let area = cells.len();
            let mut perimeter = 0;
            let occupied: HashSet<(usize, usize)> = HashSet::from_iter(cells.iter().copied());
            for (x, y) in cells {
                let sides: [(i32, i32); 4] = [
                    (-1, 0), (0, -1), (1, 0), (0, 1)
                ];
                for side in sides {
                    let xx = x.wrapping_add(side.0 as usize);
                    let yy = y.wrapping_add(side.1 as usize);
                    if !occupied.contains(&(xx, yy)) {
                        perimeter += 1;
                    }
                }
            }
            sum += area * perimeter;
            println!("region of {char} has area {area} and perimeter {perimeter} = {}", area * perimeter);
        }
    }
    sum as i64
}
/// AoC placement:
/// Time 00:47:07 rank 1328
/// Result: 821372
pub fn part2(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    let nums = get_numbers(&whole);
    let grid = get_grid(whole);
    let grid = get_grid_b(whole);
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    let mut regions = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            let c = grid[y][x];
            regions.entry(c).or_insert(vec![]).push((x, y));
        }
    }
    // subdivide regions further
    let regions_orig = regions;
    let mut regions = HashMap::new();
    for (char, cells) in regions_orig.iter() {
        let occupied: HashSet<(usize, usize)> = HashSet::from_iter(cells.iter().copied());
        let mut pooled_cells = HashMap::new();
        let mut pools = HashMap::new();
        for (i, cell) in cells.iter().enumerate() {
            pooled_cells.insert(*cell, i);
        }
        for i in 0..cells.len() {
            pools.insert(i, vec![cells[i]]);
        }
        for (i, (x, y)) in cells.iter().enumerate() {
            let my_pool_i = *pooled_cells.get(&(*x, *y)).unwrap();
            let sides: [(i32, i32); 4] = [
                (-1, 0), (0, -1), (1, 0), (0, 1)
            ];
            for side in sides {
                let xx = x.wrapping_add(side.0 as usize);
                let yy = y.wrapping_add(side.1 as usize);
                if occupied.contains(&(xx, yy)) {
                    // move all cells from that pool into this one's.
                    let other_pool_i = *pooled_cells.get(&(xx, yy)).unwrap();
                    if other_pool_i == my_pool_i {
                        continue;
                    }
                    let other_pool = pools.get(&other_pool_i).unwrap().clone();
                    // empty the other pool
                    pools.remove(&other_pool_i);
                    
                    // set the cells from that pool, to my pool
                    for (xx, yy) in other_pool.iter().copied() {
                        *pooled_cells.get_mut(&(xx, yy)).unwrap() = my_pool_i;
                    }

                    // extend my pool
                    let my_pool = pools.get_mut(&my_pool_i).unwrap();
                    my_pool.extend(other_pool);
                }
            }
        }
        // each pool now has a region
        regions.insert(char, pools.into_values().collect::<Vec<Vec<(usize, usize)>>>());
    }

    for (&char, cell_regions) in regions.iter() {
        for cells in cell_regions {
            let char = char::from_u32(*char as u32).unwrap();
            let area = cells.len();
            let mut perimeter = HashMap::new();
            let occupied: HashSet<(usize, usize)> = HashSet::from_iter(cells.iter().copied());
            for (x, y) in cells {
                let sides: [(i32, i32); 4] = [
                    (-1, 0), (0, -1), (1, 0), (0, 1)
                ];
                for side in sides {
                    let xx = x.wrapping_add(side.0 as usize);
                    let yy = y.wrapping_add(side.1 as usize);
                    if !occupied.contains(&(xx, yy)) {
                        perimeter.entry((xx, yy)).or_insert(vec![]).push(side);
                    }
                }
            }
            let mut found_sides = 0;
            while !perimeter.is_empty() {
                let (cell, sides) = perimeter.iter_mut().next().unwrap();
                let cell = *cell;
                if sides.is_empty() {
                    perimeter.remove(&cell);
                    continue;
                }
                let side = sides.pop().unwrap();
                // we have a side. now remove it from elsewhere.
                found_sides += 1;
                let sides = [(side.1, side.0), (-side.1, -side.0)];
                for (xd, yd) in sides {
                    let (mut x, mut y) = cell;
                    x = x.wrapping_add(xd as usize);
                    y = y.wrapping_add(yd as usize);
                    while let Some(candidate_sides) = perimeter.get_mut(&(x, y)) {
                        if let Some((i, _)) = candidate_sides.iter().enumerate().find(|(i, s)| **s == side) {
                            candidate_sides.swap_remove(i);
                        } else {
                            break;
                        }
                        x = x.wrapping_add(xd as usize);
                        y = y.wrapping_add(yd as usize);
                    }
                }
            }
            sum += area * found_sides;
            println!("region of {char} has area {area} and {found_sides} sides = {}", area * found_sides);
        }
    }
    sum as i64
}