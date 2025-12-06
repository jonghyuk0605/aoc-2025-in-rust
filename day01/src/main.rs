const EXAMPLE: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() {
    println!("Advent of Code 2025 - Day 01");

    let input = include_str!("../input.txt");

    println!("Part 1 (example): {}", part1(EXAMPLE));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (example): {}", part2(EXAMPLE));
    println!("Part 2: {}", part2(input));
}

fn parse(input: &str) -> Vec<(char, i64)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let dir = line.chars().next().unwrap();
            let num: i64 = line[1..].parse().unwrap();
            (dir, num)
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let moves = parse(input);
    // dbg!(&moves);
    let mut pos: i64 = 50;
    let mut count: i64 = 0;
    for (dir, num) in moves {
        if dir == 'L' {
            pos -= num;
        } else {
            pos += num;
        }
        pos = pos % 100;
        if pos == 0 {
            count += 1;
        }
    }
    count
}

fn part2(input: &str) -> i64 {
    let moves = parse(input);
    // dbg!(&moves);
    let mut pos: i64 = 50;
    let mut count: i64 = 0;
    for (dir, num) in moves {
        count += num / 100;
        let num_res = num % 100;
        if num_res == 0 { continue; }
        if dir == 'L' {
            if pos > 0 && pos <= num_res {
                count += 1;
            }
            pos = (pos - num_res + 100) % 100;
        } else {
            if pos + num_res > 99 {
                count += 1;
            }
            pos = (pos + num_res) % 100;
        }
    }
    count
}
