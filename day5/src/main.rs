use std::{convert::Infallible, str::FromStr};

#[derive(Debug)]
struct Puzzle {
    ranges: Vec<(usize, usize)>,
    ids: Vec<usize>,
}

impl FromStr for Puzzle {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (r, i) = s.split_once("\n\n").unwrap();
        let ranges = r
            .lines()
            .map(|l| l.split_once('-').unwrap())
            .map(|(s, e)| (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap()))
            .collect::<Vec<_>>();
        let ids = i
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Ok(Puzzle { ranges, ids })
    }
}

impl Puzzle {
    fn in_range(&self, id: usize) -> bool {
        self.ranges.iter().any(|(s, e)| id >= *s && id <= *e)
    }
}

fn part1(puzzle: &Puzzle) -> usize {
    puzzle.ids.iter().filter(|id| puzzle.in_range(**id)).count()
}

fn overlap(range1: (usize, usize), range2: (usize, usize)) -> bool {
    !(range1.1 < range2.0 || range1.0 > range2.1)
}

fn merge(range1: (usize, usize), range2: (usize, usize)) -> (usize, usize) {
    (range1.0.min(range2.0), range1.1.max(range2.1))
}

fn merge_ranges(ranges: Vec<(usize, usize)>, new_range: (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = ranges.clone();
    for i in 0..res.len() {
        if overlap(new_range, *res.get(i).unwrap()) {
            let overlap_range = res.remove(i);
            let merged_range = merge(new_range, overlap_range);
            return merge_ranges(res, merged_range);
        }
    }
    res.push(new_range);
    res
}

fn part2(puzzle: &Puzzle) -> usize {
    let mut ranges = Vec::new();
    for r in &puzzle.ranges {
        ranges = merge_ranges(ranges, *r);
    }
    ranges.iter().map(|(s, e)| *e - *s + 1).sum()
}

fn main() {
    let input = include_str!("../input");
    let puzzle = input.parse::<Puzzle>().unwrap();
    println!("Part 1 = {}", part1(&puzzle));
    println!("Part 2 = {}", part2(&puzzle));
}
