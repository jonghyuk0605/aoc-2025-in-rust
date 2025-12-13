use std::collections::HashMap;

const EXAMPLE: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

fn main() {
    println!("Advent of Code 2025 - Day 10");

    let input = include_str!("../input.txt");

    println!("Part 1 (example): {}", part1(EXAMPLE));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (example): {}", part2(EXAMPLE));
    println!("Part 2: {}", part2(input));
}

struct Machine {
    indicators: Vec<bool>,        // [.##.] -> [false, true, true, false]
    toggles: Vec<Vec<usize>>,     // (3) (1,3) ... -> toggle switch indices
    joltage: Vec<i64>,            // {3,5,4,7} -> joltage values
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            // Extract indicators from [...] - '.' = false, '#' = true
            let pattern_end = line.find(']').unwrap();
            let indicators: Vec<bool> = line[1..pattern_end]
                .chars()
                .map(|c| c == '#')
                .collect();

            // Extract toggles from (...) 
            let toggles: Vec<Vec<usize>> = line[pattern_end + 1..]
                .split('(')
                .filter(|s| s.contains(')'))
                .map(|s| {
                    let end = s.find(')').unwrap();
                    s[..end]
                        .split(',')
                        .map(|n| n.trim().parse().unwrap())
                        .collect()
                })
                .collect();

            // Extract joltage from {...}
            let brace_start = line.find('{').unwrap();
            let brace_end = line.find('}').unwrap();
            let joltage: Vec<i64> = line[brace_start + 1..brace_end]
                .split(',')
                .map(|n| n.trim().parse().unwrap())
                .collect();

            Machine { indicators, toggles, joltage }
        })
        .collect()
}

fn f(machine: &Machine) -> i64 {
    let indicators = &machine.indicators;
    let toggles = &machine.toggles;
    let mut states = HashMap::new();
    // # of indicators is small enough to use a binary representation
    let end_state = indicators.iter().enumerate().filter(|(_, &v)| v).fold(0, |acc, (i, _)| acc | (1 << i));
    states.insert(0, 0);
    for t in toggles {
        let mut new_states = states.clone();
        for (state, count) in states.iter() {
            let mut new_state = *state;
            for idx in t {
                if new_state & (1 << idx) == 0 {
                    new_state += 1 << idx;
                } else {
                    new_state -= 1 << idx;
                }
            }
            if !new_states.contains_key(&new_state) || new_states[&new_state] > *count + 1 {
                new_states.insert(new_state, *count + 1);
            }
        }
        states = new_states;
    }
    *states.get(&end_state).unwrap()
}

fn part1(input: &str) -> i64 {
    let machines = parse(input);
    machines.iter().map(|m| f(m)).sum()
}

use good_lp::{variables, constraint, default_solver, SolverModel, Solution, Variable, Expression, variable};

fn g(machine: &Machine) -> i64 {
    let toggles = &machine.toggles;
    let joltage = &machine.joltage;

    let mut vars = variables!();
    let v: Vec<Variable> = (0..toggles.len())
        .map(|_| vars.add(variable().integer().min(0))) // lower bound 0
        .collect();

    let objective: Expression = v.iter().copied().sum();
    let mut problem = vars.minimise(objective).using(default_solver);

    for i in 0..joltage.len() {
        let lhs: Expression = toggles.iter().enumerate().filter(|(_, t)| t.contains(&i)).map(|(idx, _)| v[idx]).sum();
        problem = problem.with(constraint!(lhs == joltage[i] as f64));
    }
    let X = problem.solve().unwrap();
    v.iter().map(|v| X.value(*v)).sum::<f64>().round() as i64
}

fn part2(input: &str) -> i64 {
    let machines = parse(input);
    machines.iter().map(|m| g(m)).sum()
}
