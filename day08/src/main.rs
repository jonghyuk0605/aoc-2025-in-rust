#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
    
    fn euclidean_squared(&self, other: &Point) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

struct Groups {
    parent: Vec<usize>
}

impl Groups {
    fn new(n: usize) -> Self {
        Self { parent: (0..n).collect() }
    }

    fn union(&mut self, i: usize, j: usize) {
        let pi = self.find(i);
        let pj = self.find(j);
        if pi != pj {
            self.parent[pi] = pj;
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            self.parent[i] = self.find(self.parent[i]);
            self.parent[i]
        }
    }
}

const EXAMPLE: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

fn main() {
    println!("Advent of Code 2025 - Day 08");

    let input = include_str!("../input.txt");

    println!("Part 1 (example): {}", part1(EXAMPLE, 10, 3));
    println!("Part 1: {}", part1(input, 1000, 3));

    println!("Part 2 (example): {}", part2(EXAMPLE));
    println!("Part 2: {}", part2(input));
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Point::new(nums[0], nums[1], nums[2])
        })
        .collect()
}

fn part1(input: &str, n: usize, k: usize) -> i64 {
    let points = parse(input);
    let mut distances = Vec::new();
    for i in 0..points.len() {
        for j in (i+1)..points.len() {
            distances.push((points[i].euclidean_squared(&points[j]), i, j));
        }
    }
    distances.sort_by_key(|(d, _, _)| *d);
    let mut g = Groups::new(points.len());
    for idx in 0..n {
        let (_, i, j) = distances[idx];
        if g.find(i) != g.find(j) {
            g.union(i, j);
        }
    }
    let mut counts = vec![0; points.len()];
    for i in 0..points.len() {
        counts[g.find(i)] += 1;
    }
    counts.sort_by_key(|c| -*c);
    counts.iter().take(k).product()
}

fn part2(input: &str) -> i64 {
    let points = parse(input);
    let mut distances = Vec::new();
    for i in 0..points.len() {
        for j in (i+1)..points.len() {
            distances.push((points[i].euclidean_squared(&points[j]), i, j));
        }
    }
    distances.sort_by_key(|(d, _, _)| *d);
    let mut g = Groups::new(points.len());
    for d in distances {
        let (_, i, j) = d;
        if g.find(i) != g.find(j) {
            g.union(i, j);
        }

        let p0 = g.find(0);
        let mut flag = false;
        for i in 1..points.len() {
            if g.find(i) != p0 {
                flag = true;
                break;
            }
        }
        if !flag {
            return points[i].x * points[j].x
        }
    }
    -1
}
