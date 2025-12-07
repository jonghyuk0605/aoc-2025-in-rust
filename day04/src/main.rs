const EXAMPLE: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn main() {
    println!("Advent of Code 2025 - Day 04");

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

fn neighbors(i: usize, j: usize, r: usize, c: usize) -> impl Iterator<Item = (usize, usize)> {
    let dirs: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];
    dirs.into_iter().filter_map(move |(di, dj)| {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;
        if ni >= 0 && ni < r as i32 && nj >= 0 && nj < c as i32 {
            Some((ni as usize, nj as usize))
        } else {
            None
        }
    })
}

fn part1(input: &str) -> i64 {
    let grid = parse(input);
    let r = grid.len();
    let c = grid[0].len();

    let mut count = 0;
    for i in 0..r {
        for j in 0..c {
            if grid[i][j] == '@' {
                let adj_count = neighbors(i, j, r, c).filter(|(ni, nj)| grid[*ni][*nj] == '@').count();
                if adj_count < 4 {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part2(input: &str) -> i64 {
    let mut grid = parse(input);
    let r = grid.len();
    let c = grid[0].len();

    let mut count = 0;
    loop {
        let mut current_count = 0;
        for i in 0..r {
            for j in 0..c {
                if grid[i][j] == '@' {
                    let adj_count = neighbors(i, j, r, c).filter(|(ni, nj)| grid[*ni][*nj] != '.').count();
                    if adj_count < 4 {
                        current_count += 1;
                        grid[i][j] = 'x';
                    }
                }
            }
        }
        if current_count == 0 {
            break;
        }
        grid.iter_mut()
            .flatten()
            .for_each(|cell| {
                if *cell == 'x' {
                    *cell = '.';
                }
            });
        count += current_count;
    }
    count
}

