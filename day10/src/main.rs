use std::{collections::HashSet, convert::Infallible, str::FromStr};

use pathfinding::matrix::Matrix;

#[derive(Debug)]
struct Machine {
    lights: Matrix<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl FromStr for Machine {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp = s.split_ascii_whitespace();

        let l = sp.next().unwrap();
        let bl = l[1..(l.len() - 1)]
            .chars()
            .map(|c| c == '#')
            .collect::<Vec<_>>();
        let lights = Matrix::from_vec(1, bl.len(), bl).unwrap();

        let mut buttons = Vec::new();
        let mut joltage = Vec::new();

        for c in sp {
            if c.starts_with('(') {
                let b = c[1..(c.len() - 1)]
                    .split(',')
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                buttons.push(b);
            } else if c.starts_with('{') {
                joltage = c[1..(c.len() - 1)]
                    .split(',')
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
            } else {
                panic!();
            }
        }
        Ok(Machine {
            lights,
            buttons,
            joltage,
        })
    }
}

impl Machine {
    fn compute_new_state(state: &Matrix<bool>, button: &[usize]) -> Matrix<bool> {
        let mut res = state.clone();
        for b in button {
            *res.get_mut((0, *b)).unwrap() ^= true;
        }
        res
    }

    fn solve_part1(&self) -> usize {
        let mut start_states = vec![Matrix::from_fn(1, self.lights.columns, |_| false)];
        let mut future_states = Vec::new();
        let mut visited_states = HashSet::new();
        let mut res = 1;

        loop {
            for state in start_states {
                for button in &self.buttons {
                    let next_state = Machine::compute_new_state(&state, button);
                    if next_state == self.lights {
                        return res;
                    }
                    if visited_states.insert(next_state.clone()) {
                        future_states.push(next_state);
                    }
                }
            }
            start_states = future_states;
            future_states = Vec::new();
            res += 1;
        }
    }
}

fn part1(machines: &[Machine]) -> usize {
    machines.iter().map(|m| m.solve_part1()).sum()
}

#[test]
fn test_part1() {
    let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    let machines = input
        .lines()
        .map(|l| l.parse::<Machine>().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(part1(&machines), 7);
}

fn main() {
    let input = include_str!("../input");
    let machines = input
        .lines()
        .map(|l| l.parse::<Machine>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1 = {}", part1(&machines));
}
