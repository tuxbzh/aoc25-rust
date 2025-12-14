use std::{convert::Infallible, str::FromStr};

use pathfinding::matrix::Matrix;

#[derive(Debug)]
struct Area {
    size: (usize, usize),
    num_presents: Vec<usize>,
}

impl FromStr for Area {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dim, pre) = s.split_once(':').unwrap();
        let (dx, dy) = dim.split_once('x').unwrap();
        let num_presents = pre
            .split_ascii_whitespace()
            .map(|sp| sp.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Ok(Area {
            size: (dx.parse().unwrap(), dy.parse().unwrap()),
            num_presents,
        })
    }
}

#[derive(Debug)]
struct Puzzle {
    presents: Vec<Matrix<char>>,
    areas: Vec<Area>,
}

impl FromStr for Puzzle {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut presents = Vec::new();
        let mut areas = Vec::new();

        let part = s.split("\n\n");
        for p in part {
            let mut l = p.lines();
            let fl = l.next().unwrap();
            if fl.ends_with(':') {
                let present = Matrix::from_rows(l.map(|ll| ll.chars())).unwrap();
                presents.push(present);
            } else {
                areas.push(fl.parse::<Area>().unwrap());
                areas.extend(l.map(|ll| ll.parse().unwrap()));
            }
        }
        Ok(Puzzle { presents, areas })
    }
}

fn part1(puzzle: &Puzzle) -> usize {
    let area_presents = puzzle
        .presents
        .iter()
        .map(|p| p.values().filter(|v| **v == '#').count())
        .collect::<Vec<_>>();

    let mut res = 0;

    for area in &puzzle.areas {
        print!("Area {}x{} => ", area.size.0, area.size.1);
        if area.size.0 * area.size.1 >= 9 * area.num_presents.iter().sum::<usize>() {
            res += 1;
            println!("large enough");
            continue;
        }
        if area.size.0 * area.size.1
            < area
                .num_presents
                .iter()
                .zip(&area_presents)
                .map(|(n, a)| *n * *a)
                .sum::<usize>()
        {
            println!("too small");
            continue;
        }
        panic!("?");
    }

    res
}

fn main() {
    let input = include_str!("../input");
    let puzzle = input.parse::<Puzzle>().unwrap();

    println!("Part 1 = {}", part1(&puzzle));
}
