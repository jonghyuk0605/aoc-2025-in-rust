# Advent of Code 2025 in Rust

https://adventofcode.com/2025

## Project Structure

This is a Cargo workspace monorepo with each day as a separate package:

```
aoc-2025-in-rust/
├── Cargo.toml       # Workspace root
├── day01/           # Day 01 solution
├── day02/           # Day 02 solution
└── ...
```

## Running Solutions

Run a specific day's solution:

```bash
cargo run -p day01
cargo run -p day02
```

Run in release mode (faster execution):

```bash
cargo run -p day01 --release
```

## Adding a New Day

1. Create the directory and files:
   ```bash
   mkdir -p dayXX/src
   ```

2. Create `dayXX/Cargo.toml`:
   ```toml
   [package]
   name = "dayXX"
   version = "0.1.0"
   edition.workspace = true

   [dependencies]
   ```

3. Create `dayXX/src/main.rs` with your solution

4. Add `"dayXX"` to the `members` array in the root `Cargo.toml`

## Input Files

Place your puzzle input in each day's directory as `input.txt`, then read it with:

```rust
let input = include_str!("../input.txt");
```
