const EXAMPLE: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

fn main() {
    println!("Advent of Code 2025 - Day 06");

    let input = include_str!("../input.txt");

    println!("Part 1 (example): {}", part1(EXAMPLE));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (example): {}", part2(EXAMPLE));
    println!("Part 2: {}", part2(input));
}

fn parse(input: &str) -> (Vec<char>, Vec<Vec<i64>>) {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    
    // Last line: operators
    let ops: Vec<char> = lines.last().unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    
    // Other lines: numbers (parse rows, then transpose to columns)
    let rows: Vec<Vec<i64>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();
    
    // Transpose: rows -> columns (stacks)
    let num_cols = rows[0].len();
    let stacks: Vec<Vec<i64>> = (0..num_cols)
        .map(|col| rows.iter().map(|row| row[col]).collect())
        .collect();
    
    (ops, stacks)
}

fn part1(input: &str) -> i64 {
    let (ops, stacks) = parse(input);
    // dbg!(&ops, &stacks);
    assert_eq!(ops.len(), stacks.len());
    let mut result = 0;
    for i in 0..ops.len() {
        let op = ops[i];
        let stack = &stacks[i];
        if op == '+' {
            result += stack.iter().sum::<i64>();
        } else {
            result += stack.iter().product::<i64>();
        }
    }
    result
}

fn parse2(input: &str) -> (Vec<char>, Vec<Vec<i64>>) {
    let mut lines: Vec<Vec<char>> = input.lines()
    .filter(|l| !l.is_empty())
    .map(|line| line.chars().collect()).collect();

    let last_line = lines.pop().unwrap();
    // Last line: operators
    let ops: Vec<char> = last_line.into_iter().filter(|c| !c.is_whitespace()).rev().collect();

    let max_cols = lines.iter().map(|line| line.len()).max().unwrap();
    let mut stacks: Vec<Vec<i64>> = Vec::new();
    let mut stack: Vec<i64> = Vec::new();
    for c in (0..max_cols).rev() {
        let num_stack: Vec<char> = (0..lines.len())
        .filter_map(|r| (
            if c >= lines[r].len() || lines[r][c].is_whitespace() { None }
            else { Some(lines[r][c]) }
        )).collect();
        if num_stack.is_empty() {
            stacks.push(stack);
            stack = Vec::new();
        } else {
            let num: i64 = num_stack.into_iter().collect::<String>().parse().unwrap();
            stack.push(num);
        }
    }
    if !stack.is_empty() {
        stacks.push(stack);
    }

    (ops, stacks)
}

fn part2(input: &str) -> i64 {
    let (ops, stacks) = parse2(input);
    // dbg!(&ops, &stacks);
    assert_eq!(ops.len(), stacks.len());
    let mut result = 0;
    for i in 0..ops.len() {
        let op = ops[i];
        let stack = &stacks[i];
        if op == '+' {
            result += stack.iter().sum::<i64>();
        } else {
            result += stack.iter().product::<i64>();
        }
    }
    result
}

