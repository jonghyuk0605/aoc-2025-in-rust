use std::panic::AssertUnwindSafe;

const EXAMPLE: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

fn main() {
    println!("Advent of Code 2025 - Day 09");

    let input = include_str!("../input.txt");

    println!("Part 1 (example): {}", part1(EXAMPLE));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (example): {}", part2(EXAMPLE));
    println!("Part 2: {}", part2(input));
}

fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn compress(points: &[(i64, i64)]) -> (Vec<(usize, usize)>, usize, usize) {
    use std::collections::HashMap;
    
    let mut xs: Vec<i64> = points.iter().map(|(x, _)| *x).collect();
    let mut ys: Vec<i64> = points.iter().map(|(_, y)| *y).collect();
    
    xs.sort();
    xs.dedup();
    ys.sort();
    ys.dedup();
    
    let x_map: HashMap<i64, usize> = xs.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let y_map: HashMap<i64, usize> = ys.iter().enumerate().map(|(i, &y)| (y, i)).collect();
    
    (points.iter().map(|(x, y)| (x_map[x] + 1, y_map[y] + 1)).collect(), xs.len() + 2, ys.len() + 2)
}

fn area(p1: &(i64, i64), p2: &(i64, i64)) -> i64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1)
}

fn part1(input: &str) -> i64 {
    let points = parse(input);
    // dbg!(&points);
    (0..points.len()).flat_map(|i| (i+1..points.len()).map(move |j|  (i, j)))
    .map(|(i, j)| area(&points[i], &points[j])).max().unwrap()    
}

fn part2(input: &str) -> i64 {
    let points = parse(input);
    let (compressed, r, c) = compress(&points);
    let mut map = vec![vec![0i32; c]; r];
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        let (x1, y1) = compressed[i];
        let (x2, y2) = compressed[j];
        if x1 == x2 {
            let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            for y in y1..=y2 {
                map[y][x1] = 1;
            }
        }
        else if y1 == y2 {
            let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            for x in x1..=x2 {
                map[y1][x] = 1;
            }
        }
        else {
            panic!("Error: diagonal line");
        }
    }
    let mut q = vec![(0i32, 0i32)];
    while !q.is_empty() {
        let (x, y) = q.pop().unwrap();
        map[x as usize][y as usize] = -1;
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < r as i32 && ny >= 0 && ny < c as i32 && map[nx as usize][ny as usize] == 0 {
                q.push((nx, ny));
            }
        }
    }
    let mut max_area = 0;
    for i in 0..compressed.len() {
        for j in (i + 1)..compressed.len() {
            let (x1, y1) = compressed[i];
            let (x2, y2) = compressed[j];
            let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            let mut flag = true;
            for x in x1..=x2 {
                for y in y1..=y2 {
                    if map[y][x] == -1 {
                        flag = false;
                        break;
                    }
                }
                if !flag {
                    break;
                }
            }
            if flag {
                let area = area(&points[i], &points[j]);
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }
    max_area
}

