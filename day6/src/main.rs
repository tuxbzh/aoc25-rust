use std::{convert::Infallible, str::FromStr};

use pathfinding::matrix::Matrix;

#[derive(Debug)]
struct Puzzle {
    num: Matrix<usize>,
    ops: Vec<char>,
}

impl FromStr for Puzzle {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().collect::<Vec<_>>();
        let ops_line = lines.pop().unwrap();
        let num = Matrix::from_rows(
            lines
                .into_iter()
                .map(|l| l.split_ascii_whitespace())
                .map(|c| c.map(|cc| cc.parse::<usize>().unwrap())),
        )
        .unwrap();
        let ops = ops_line
            .split_ascii_whitespace()
            .map(|s| s.chars().next().unwrap())
            .collect::<Vec<_>>();
        Ok(Puzzle { num, ops })
    }
}

impl Puzzle {
    fn solve_part1(&self) -> usize {
        self.num
            .column_iter()
            .zip(&self.ops)
            .map(|(col, op)| match op {
                '+' => col.iter().map(|n| **n).sum::<usize>(),
                '*' => col.iter().map(|n| **n).reduce(|acc, e| acc * e).unwrap(),
                _ => unreachable!(),
            })
            .sum()
    }
}

#[derive(Debug)]
struct Puzzle2 {
    num: Matrix<char>,
    ops: Vec<char>,
}

impl FromStr for Puzzle2 {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().collect::<Vec<_>>();
        let ops_line = lines.pop().unwrap();
        let num = Matrix::from_rows(lines.into_iter().map(|l| l.chars())).unwrap();
        let ops = ops_line
            .split_ascii_whitespace()
            .map(|s| s.chars().next().unwrap())
            .collect::<Vec<_>>();
        Ok(Puzzle2 { num, ops })
    }
}

impl Puzzle2 {
    fn solve_part2(&self) -> usize {
        let mut res = 0;
        let cols = self.num.column_iter().rev();
        let mut ops_iter = self.ops.iter().rev();

        let mut col_nums = Vec::new();
        for mut col in cols {
            col.retain(|c| c.is_ascii_digit());
            if col.is_empty() {
                // Separator, perform the operation
                let op = ops_iter.next().unwrap();
                res += match op {
                    '+' => col_nums.iter().sum(),
                    '*' => col_nums.iter().copied().reduce(|acc, e| acc * e).unwrap(),
                    _ => unreachable!(),
                };
                col_nums = Vec::new();
                continue;
            }
            let num = col
                .iter()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .reduce(|acc, e| acc * 10 + e)
                .unwrap();

            col_nums.push(num);
        }

        let op = ops_iter.next().unwrap();
        res += match op {
            '+' => col_nums.iter().sum(),
            '*' => col_nums.iter().copied().reduce(|acc, e| acc * e).unwrap(),
            _ => unreachable!(),
        };

        res
    }
}

#[test]
fn test_part2() {
    let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
    let puzzle2 = input.parse::<Puzzle2>().unwrap();
    assert_eq!(puzzle2.solve_part2(), 3263827);
}

fn main() {
    let input = include_str!("../input");
    let puzzle = input.parse::<Puzzle>().unwrap();
    println!("Part 1 = {}", puzzle.solve_part1());
    let puzzle2 = input.parse::<Puzzle2>().unwrap();
    println!("Part 2 = {}", puzzle2.solve_part2());
}
