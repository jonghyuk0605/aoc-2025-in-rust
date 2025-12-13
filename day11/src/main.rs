const EXAMPLE1: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

const EXAMPLE2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

fn main() {
    println!("Advent of Code 2025 - Day 11");

    let input = include_str!("../input.txt");

    println!("Part 1 (example): {}", part1(EXAMPLE1));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (example): {}", part2(EXAMPLE2));
    println!("Part 2: {}", part2(input));
}

use std::collections::HashMap;

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (node, targets) = line.split_once(": ").unwrap();
            let targets: Vec<&str> = targets.split_whitespace().collect();
            (node, targets)
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let graph = parse(input);
    // Collect all unique node names (including targets)
    let mut nodes = std::collections::BTreeSet::new();
    for (node, targets) in &graph {
        nodes.insert(*node);
        for &target in targets {
            nodes.insert(target);
        }
    }
    let node_to_idx: HashMap<&str, usize> = nodes.iter().enumerate().map(|(i, n)| (*n, i)).collect();
    let mut indegree = vec![0; nodes.len()];
    let mut idx_targets = HashMap::new();
    for (node, targets) in &graph {
        let u = node_to_idx[node];
        let mut current_targets = Vec::new();
        for target in targets {
            let v = node_to_idx[target];
            indegree[v] += 1;
            current_targets.push(v);
        }
        idx_targets.insert(u, current_targets);
    }
    let mut queue = Vec::new();
    for i in 0..nodes.len() {
        if indegree[i] == 0 {
            queue.push(i);
        }
    }
    let mut counts = vec![0; nodes.len()];
    counts[node_to_idx[&"you"]] = 1;
    while !queue.is_empty() {
        let u = queue.pop().unwrap();
        for &v in idx_targets.get(&u).unwrap_or(&vec![]).iter() {
            counts[v] += counts[u];
            indegree[v] -= 1;
            if indegree[v] == 0 { queue.push(v); }
        }
    }
    counts[node_to_idx[&"out"]]
}

fn f(graph: &HashMap<&str, Vec<&str>>, start_node: &str, end_node: &str) -> i64 {
    // Collect all unique node names (including targets)
    let mut nodes = std::collections::BTreeSet::new();
    for (node, targets) in graph.iter() {
        nodes.insert(*node);
        for &target in targets {
            nodes.insert(target);
        }
    }
    let node_to_idx: HashMap<&str, usize> = nodes.iter().enumerate().map(|(i, n)| (*n, i)).collect();
    let mut indegree = vec![0; nodes.len()];
    let mut idx_targets = HashMap::new();
    for (node, targets) in graph.iter() {
        let u = node_to_idx[node];
        let mut current_targets = Vec::new();
        for target in targets {
            let v = node_to_idx[target];
            indegree[v] += 1;
            current_targets.push(v);
        }
        idx_targets.insert(u, current_targets);
    }
    let mut queue = Vec::new();
    for i in 0..nodes.len() {
        if indegree[i] == 0 {
            queue.push(i);
        }
    }
    let mut counts = vec![0; nodes.len()];
    counts[node_to_idx[start_node]] = 1;
    while !queue.is_empty() {
        let u = queue.pop().unwrap();
        for &v in idx_targets.get(&u).unwrap_or(&vec![]).iter() {
            counts[v] += counts[u];
            indegree[v] -= 1;
            if indegree[v] == 0 { queue.push(v); }
        }
    }
    counts[node_to_idx[end_node]]
}

fn part2(input: &str) -> i64 {
    let graph = parse(input);
    let x1 = f(&graph, "svr", "fft");
    let x2 = f(&graph, "fft", "dac");
    let x3 = f(&graph, "dac", "out");
    let y1 = f(&graph, "svr", "dac");
    let y2 = f(&graph, "dac", "fft");
    let y3 = f(&graph, "fft", "out");
    x1*x2*x3 + y1*y2*y3
}
