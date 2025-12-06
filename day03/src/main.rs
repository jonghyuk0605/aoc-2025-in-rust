const EXAMPLE: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

fn main() {
    println!("Advent of Code 2025 - Day 03");

    let input = include_str!("../input.txt");

    println!("Part 1 (example): {}", part1(EXAMPLE));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (example): {}", part2(EXAMPLE));
    println!("Part 2: {}", part2(input));
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn f(row: &[u8]) -> i64 {
    let mut ret: i64 = 0;
    let mut b = row[row.len() - 1] as i64;
    for i in (0..row.len() - 1).rev() {
        let a = row[i] as i64;
        if a * 10 + b > ret {
            ret = a * 10 + b;
        }
        if a > b {
            b = a;
        }
    }
    ret
}

fn part1(input: &str) -> i64 {
    let grid = parse(input);
    // dbg!(&grid);
    grid.iter().map(|row| f(row)).sum()
}

fn g(row: &[u8], n: usize) -> i64 { // f is a n=2 special case
    let mut ret: i64 = 0;
    let mut b_vec: Vec<i64> = vec![0; n - 1];
    b_vec[0] = row[row.len() - 1] as i64;
    for i in (0..row.len() - 1).rev() {
        let a = row[i] as i64;
        for l in (1..n).rev() {
            if l > row.len() - 1 - i { continue; }
            let b = b_vec[l - 1];
            let x = a * 10_i64.pow(l as u32) + b;
            if x > ret {
                ret = x;
            }
            if l < n - 1 && x > b_vec[l] {
                b_vec[l] = x;
            }
        }
        if a > b_vec[0] {
            b_vec[0] = a;
        }
    }
    // println!("{}", ret);
    ret
}

fn part2(input: &str) -> i64 {
    let grid = parse(input);
    // dbg!(&grid);
    grid.iter().map(|row| g(row, 12)).sum()
}

