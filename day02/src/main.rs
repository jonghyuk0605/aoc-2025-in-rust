use std::cmp;


const EXAMPLE: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
";

fn main() {
    println!("Advent of Code 2025 - Day 02");

    let input = include_str!("../input.txt");

    println!("Part 1 (example): {}", part1(EXAMPLE));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (example): {}", part2(EXAMPLE));
    println!("Part 2: {}", part2(input));
}

fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .replace('\n', "")
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (a, b) = s.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn f(r: &(i64, i64)) -> i64 {
    let (a, b) = r;
    let mut tens: i64 = 10;
    let mut sums: i64 = 0;
    loop {
        let base = tens + 1;
        if base * (tens / 10) > *b { break; }
        let lb: i64 = cmp::max(tens / 10, (*a + base - 1) / base);
        let ub: i64 = cmp::min(tens - 1, *b / base);
        if lb <= ub {
            sums += (ub + lb) * (ub - lb + 1) / 2 * base;
        }
        tens *= 10;
    }
    sums
}

fn part1(input: &str) -> i64 {
    let ranges = parse(input);
    // dbg!(&ranges);
    ranges.iter().map(|r| f(r)).sum()
}

fn h(l: u32, r: u32) -> (i64, i64) {
    let tens = 10_i64.pow(l as u32);
    let mut base = 1;
    for _ in 1..r {
        base = base * tens + 1;
    }
    (base, tens)
}

fn g(r: &(i64, i64)) -> i64 {
    let (a, b) = r;
    let mut l: u32 = 1;
    let mut sums = 0;
    loop {
        let mut r: u32 = 2;
        let (base, tens) = h(l, r);
        if base > *b { break; }
        loop {
            let (base, tens) = h(l, r);
            if base > *b { break; }
            let lb: i64 = cmp::max(tens / 10, (*a + base - 1) / base);
            let ub: i64 = cmp::min(tens - 1, *b / base);
            if lb <= ub {
                sums += ((ub + lb) * (ub - lb + 1) / 2 - g(&(lb, ub))) * base;
            }
            r += 1;
        }
        l += 1;
    }
    sums
}

fn part2(input: &str) -> i64 {
    let ranges = parse(input);
    // dbg!(&ranges);
    ranges.iter().map(|r| g(r)).sum()
}
