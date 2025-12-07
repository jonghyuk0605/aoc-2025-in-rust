const EXAMPLE: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

use std::collections::{BTreeSet, BTreeMap};

fn main() {
    println!("Advent of Code 2025 - Day 07");

    let input = include_str!("../input.txt");

    println!("Part 1 (example): {}", part1(EXAMPLE));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (example): {}", part2(EXAMPLE));
    println!("Part 2: {}", part2(input));
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn part1(input: &str) -> i64 {
    let grid = parse(input);
    let cols: Vec<Vec<usize>> = (0..grid[0].len()).map(|c| {
        (0..grid.len()).filter_map(|r| if grid[r][c] == '^' { Some(r) } else { None }).collect()
    }).collect();
    // dbg!(&cols);
    let start_col = grid[0].iter().position(|&c| c == 'S').unwrap();
    if cols[start_col].is_empty() {
        return 0;
    }
    let mut pq: BTreeSet<(usize, usize)> = BTreeSet::new();
    pq.insert((cols[start_col][0], start_col));
    let mut count = 0;
    while let Some((r, c)) = pq.pop_first() {
        count += 1;
        // There are no adjacent ^ and also no tachyon going out of bounds
        for cc in [c - 1, c + 1] {
            for next_r in cols[cc].iter() {
                if *next_r > r {
                    pq.insert((*next_r, cc));
                    break;
                }
            }
        }
    }
    count
}

fn part2(input: &str) -> i64 {
    let grid = parse(input);
    let cols: Vec<Vec<usize>> = (0..grid[0].len()).map(|c| {
        (0..grid.len()).filter_map(|r| if grid[r][c] == '^' { Some(r) } else { None }).collect()
    }).collect();
    // dbg!(&cols);
    let start_col = grid[0].iter().position(|&c| c == 'S').unwrap();
    if cols[start_col].is_empty() {
        return 0;
    }
    let mut pq: BTreeMap<(usize, usize), i64> = BTreeMap::new();
    pq.insert((cols[start_col][0], start_col), 1);
    let mut count = 0;
    while let Some(((r, c), num_paths)) = pq.pop_first() {
        // There are no adjacent ^ and also no tachyon going out of bounds
        for cc in [c - 1, c + 1] {
            let mut flag = false;
            for next_r in cols[cc].iter() {
                if *next_r > r {
                    if !pq.contains_key(&(*next_r, cc)) {
                        pq.insert((*next_r, cc), num_paths);
                    } else {
                        *pq.get_mut(&(*next_r, cc)).unwrap() += num_paths;
                    }
                    flag = true;
                    break;
                }
            }
            if !flag {
                count += num_paths;
            }
        }
    }
    count
}

