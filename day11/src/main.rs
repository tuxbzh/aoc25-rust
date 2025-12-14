use std::collections::HashMap;

use pathfinding::prelude::count_paths;

fn solve_part1(puzzle: &HashMap<String, Vec<String>>) -> usize {
    count_paths(
        "you".to_owned(),
        |n| puzzle.get(n).unwrap().iter().cloned(),
        |n| *n == "out",
    )
}

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|l| {
            let (d, o) = l.split_once(':').unwrap();
            let outs = o
                .split_ascii_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<_>>();
            (d.to_owned(), outs)
        })
        .collect::<HashMap<_, _>>()
}

fn main() {
    let input = include_str!("../input");
    let puzzle = parse_input(input);
    println!("Part 1 = {}", solve_part1(&puzzle));
}
