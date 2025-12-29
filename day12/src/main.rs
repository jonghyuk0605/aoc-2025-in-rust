const EXAMPLE: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

fn main() {
    println!("Advent of Code 2025 - Day 12");

    let input = include_str!("../input.txt");

    println!("Part 1 (example): {}", part1(EXAMPLE));
    println!("Part 1: {}", part1(input));

    // println!("Part 2 (example): {}", part2(EXAMPLE));
    // println!("Part 2: {}", part2(input));
}

type Tile = [[bool; 3]; 3];  // 3x3 grid, # = true, . = false

struct Query {
    width: usize,
    height: usize,
    num_tiles: Vec<usize>,
}

struct Input {
    tiles: Vec<Tile>,
    queries: Vec<Query>,
}

fn parse(input: &str) -> Input {
    let mut tiles = Vec::new();
    let mut queries = Vec::new();

    let blocks: Vec<&str> = input.split("\n\n").collect();
    
    for block in blocks {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        // Check if it's a tile definition (starts with "N:")
        if let Some(first_line) = block.lines().next() {
            if first_line.ends_with(':') {
                // Tile definition
                let grid_lines: Vec<&str> = block.lines().skip(1).collect();
                let mut tile: Tile = [[false; 3]; 3];
                for (r, line) in grid_lines.iter().enumerate() {
                    for (c, ch) in line.chars().enumerate() {
                        tile[r][c] = ch == '#';
                    }
                }
                tiles.push(tile);
            } else {
                // Query lines (could be multiple)
                for line in block.lines() {
                    if line.contains('x') && line.contains(':') {
                        let (dims, tile_list) = line.split_once(": ").unwrap();
                        let (w, h) = dims.split_once('x').unwrap();
                        let width = w.parse().unwrap();
                        let height = h.parse().unwrap();
                        let num_tiles: Vec<usize> = tile_list
                            .split_whitespace()
                            .map(|s| s.parse().unwrap())
                            .collect();
                        queries.push(Query { width, height, num_tiles });
                    }
                }
            }
        }
    }

    Input { tiles, queries }
}

// use good_lp::{variables, constraint, default_solver, SolverModel, Solution, Variable, Expression, variable};
use good_lp::{variables, constraint, SolverModel, Variable, Expression, variable};
use good_lp::solvers::highs::highs;

fn rotate_tile(tile: &Tile) -> Tile {
    let mut rotated = [[false; 3]; 3];
    for x in 0..3 {
        for y in 0..3 {
            rotated[y][2 - x] = tile[x][y];
        }
    }
    rotated
}

fn var_idx(tile_idx: usize, rotation: usize, x: usize, y: usize, height: usize, width: usize) -> usize {
    tile_idx * 4 * (height - 2) * (width - 2) + rotation * (height - 2) * (width - 2) + x * (width - 2) + y
}

// fn idx_to_tile_idx(idx: usize, height: usize, width: usize) -> (usize, usize, usize, usize) {
//     let tile_idx = idx / (4 * (height - 2) * (width - 2));
//     let rotation = (idx % (4 * (height - 2) * (width - 2))) / ((height - 2) * (width - 2));
//     let x = (idx % ((height - 2) * (width - 2))) / (width - 2);
//     let y = idx % (width - 2);
//     (tile_idx, rotation, x, y)
// }

// fn idx_to_tile(tiles: &[Tile], idx: usize, height: usize, width: usize) -> Tile {
//     let (tile_idx, rotation, x, y) = idx_to_tile_idx(idx, height, width);
//     let mut tile = tiles[tile_idx].clone();
//     for _ in 0..rotation {
//         tile = rotate_tile(&tile);
//     }
//     tile
// }

// fn dbg_tile(tile: &Tile) {
//     for row in tile {
//         print!("{}", row.iter().map(|&x| if x { "#" } else { "." }).collect::<String>());
//         println!();
//     }
// }

// fn dbg_lhs(tiles: &[Tile], bx: usize, by: usize, width: usize, height: usize, lhs: Vec<(usize, usize, usize, usize)>) {
//     println!("x: {}, y: {}", bx, by);
//     for (tile_idx, rotation, x, y) in lhs {
//         let tile = idx_to_tile(tiles, tile_idx, height, width);
//         println!("var : {}", var_idx(tile_idx, rotation, x, y, height, width));
//         for xx in 0..height {
//             for yy in 0..width {
//                 match (xx == bx && yy == by, x <= xx && xx < x + 3 && y <= yy && yy < y + 3) {
//                     (true, _) => print!("X"),
//                     (false, true) => {
//                         match tile[xx - x][yy - y] {
//                             true => print!("#"),
//                             false => print!("."),
//                         }
//                     }
//                     (false, false) => print!("."),
//                 }
//             }
//             println!();
//         }
//     }
// }

fn f(query: &Query, tiles: &[Tile]) -> bool {
    let height = query.height;
    let width = query.width;
    let num_tiles = &query.num_tiles;

    let num_vars = tiles.len() * 4 * (height - 2) * (width - 2); // tiles are always 3x3, some are rotation invariant but ignore it for simplicity

    let mut vars = variables!();
    let v: Vec<Variable> = (0..num_vars)
        .map(|_| vars.add(variable().binary()))
        .collect();

    println!("num_vars: {}", num_vars);
    let objective: Expression = v.iter().copied().sum();
    // let mut problem: good_lp::solvers::microlp::MicroLpProblem = vars.maximise(objective).using(default_solver);
    let mut problem = vars.maximise(objective).using(highs).set_time_limit(60.0); // 60 seconds

    for i in 0..tiles.len() {
        let mut tile_vars = Vec::new();
        // println!("tile {}:", i);
        for rotation in 0..4 {
            for x in 0..(height - 2) {
                for y in 0..(width - 2) {
                    let v_idx = var_idx(i, rotation, x, y, height, width);
                    tile_vars.push(v[v_idx]);
                    // let tile = idx_to_tile(tiles, v_idx, height, width);
                    // dbg_tile(&tile);
                }
            }
        }
        let lhs = tile_vars.iter().sum::<Expression>();
        // dbg!(&lhs, &num_tiles[i]);
        problem = problem.with(constraint!(lhs == num_tiles[i] as f64));
    }

    let mut added_vars: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; width]; height];
    for i in 0..tiles.len() {
        let mut tile = tiles[i].clone();
        for rotation in 0..4 {
            for x in 0..(height - 2) {
                for y in 0..(width - 2) {
                    for dx in 0..3 {
                        for dy in 0..3 {
                            if tile[dx][dy] {
                                added_vars[x + dx][y + dy].push(var_idx(i, rotation, x, y, height, width));
                            }
                        }
                    }
                }
            }
            tile = rotate_tile(&tile);
        }
    }
    for x in 0..height {
        for y in 0..width {
            // dbg_lhs(tiles, x, y, width, height, added_vars[x][y].iter().map(|&i| idx_to_tile_idx(i, height, width)).collect::<Vec<(usize, usize, usize, usize)>>());
            let lhs = added_vars[x][y].iter().map(|&i| v[i]).sum::<Expression>();
            problem = problem.with(constraint!(lhs <= 1_f64));
        }
    }

    match problem.solve() {
        Ok(_) => {
        // Ok(x) => {
            // dbg!(&x);
            true
        }
        Err(e) => {
            println!("Error: {:?}", e);
            false
        }
    }
}

fn part1(input: &str) -> usize {
    let data = parse(input);
    let mut count = 0;
    for (i, q) in data.queries.iter().enumerate() {
        let start = std::time::Instant::now();
        println!("query {}, width: {}, height: {}, num_tiles: {:?}", i, q.width, q.height, q.num_tiles);
        let result = f(q, &data.tiles);
        let elapsed = start.elapsed();
        println!("result: {}, time: {:.2?}", result, elapsed);
        println!("--------------------------------");
        if result {
            count += 1;
        }
        // break;
    }
    count
}

// fn part2(input: &str) -> i64 {
//     let data = parse(input);
//     0
// }
