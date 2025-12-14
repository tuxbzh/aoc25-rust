use std::collections::HashMap;

use pathfinding::prelude::count_paths;

fn solve_part1(puzzle: &HashMap<String, Vec<String>>) -> usize {
    count_paths(
        "you".to_owned(),
        |n| puzzle.get(n).unwrap_or(&Vec::new()).clone(),
        |n| *n == "out",
    )
}

fn solve_part2(puzzle: &HashMap<String, Vec<String>>) -> usize {
    let svrdac = count_paths(
        "svr".to_owned(),
        |n| puzzle.get(n).unwrap_or(&Vec::new()).clone(),
        |n| *n == "dac",
    );

    let svrfft = count_paths(
        "svr".to_owned(),
        |n| puzzle.get(n).unwrap_or(&Vec::new()).clone(),
        |n| *n == "fft",
    );

    let fftdac = count_paths(
        "fft".to_owned(),
        |n| puzzle.get(n).unwrap_or(&Vec::new()).clone(),
        |n| *n == "dac",
    );

    let dacfft = count_paths(
        "dac".to_owned(),
        |n| puzzle.get(n).unwrap_or(&Vec::new()).clone(),
        |n| *n == "fft",
    );

    let dacout = count_paths(
        "dac".to_owned(),
        |n| puzzle.get(n).unwrap_or(&Vec::new()).clone(),
        |n| *n == "out",
    );

    let fftout = count_paths(
        "fft".to_owned(),
        |n| puzzle.get(n).unwrap_or(&Vec::new()).clone(),
        |n| *n == "out",
    );

    svrdac * dacfft * fftout + svrfft * fftdac * dacout
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
    println!("Part 2 = {}", solve_part2(&puzzle));
}
