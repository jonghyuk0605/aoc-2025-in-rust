const EXAMPLE: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

fn main() {
    println!("Advent of Code 2025 - Day 05");

    let input = include_str!("../input.txt");

    println!("Part 1 (example): {}", part1(EXAMPLE));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (example): {}", part2(EXAMPLE));
    println!("Part 2: {}", part2(input));
}

fn parse(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    
    let ranges = parts[0]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    
    let numbers = parts[1]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    
    (ranges, numbers)
}

fn part1(input: &str) -> i64 {
    let (ranges, numbers) = parse(input);
    // dbg!(&ranges, &numbers);
    numbers.into_iter().filter(|n| ranges.iter().any(|(a, b)| n >= a && n <= b)).count() as i64
}

fn part2(input: &str) -> i64 {
    let (ranges, _) = parse(input);
    
    // true = start, false = end
    let mut events: Vec<(i64, i64)> = ranges
        .into_iter()
        .flat_map(|(a, b)| [(a, 1), (b + 1, -1)])
        .collect();
    
    events.sort_by_key(|(x, _)| *x);
    let mut count = 0;
    let mut last_idx = events[0].0;
    let mut acc = events[0].1;
    for e in events.into_iter().skip(1) {
        if e.0 == last_idx {
            acc += e.1;
        } else {
            if acc > 0 {
                count += e.0 - last_idx;
            }
            last_idx = e.0;
            acc += e.1;
        }
    }
    count
}

