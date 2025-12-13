use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    str::FromStr,
};

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

    fn compute_buttons_patterns(&self) -> HashMap<Matrix<bool>, Vec<Vec<Vec<usize>>>> {
        let mut res: HashMap<Matrix<bool>, Vec<Vec<Vec<usize>>>> = HashMap::new();

        for b in 1..2usize.pow(self.buttons.len() as u32) {
            // Bit b[i] indicates whether self.buttons[i] is pressed
            let mut state = Matrix::from_fn(1, self.lights.columns, |_| false);
            let mut buttons_pressed = Vec::new();
            for i in 0..self.buttons.len() {
                if b & (1usize << i) != 0 {
                    state = Machine::compute_new_state(&state, self.buttons.get(i).unwrap());
                    buttons_pressed.push(self.buttons.get(i).unwrap().clone());
                }
            }
            res.entry(state).or_default().push(buttons_pressed);
        }

        res
    }

    fn solve_part2_helper(
        &self,
        patterns: &HashMap<Matrix<bool>, Vec<Vec<Vec<usize>>>>,
        objective: &Vec<usize>,
        cache: &mut HashMap<Vec<usize>, Option<usize>>,
    ) -> Option<usize> {
        if let Some(r) = cache.get(objective) {
            return *r;
        }

        if objective.iter().all(|i| *i == 0) {
            return Some(0);
        }

        let pattern = Matrix::from_fn(1, objective.len(), |(_, c)| {
            objective.get(c).unwrap() % 2 == 1
        });

        let mut solutions = Vec::new();

        let buttons_comb_opt = patterns.get(&pattern);

        if let Some(buttons_comb) = buttons_comb_opt {
            solutions.extend(buttons_comb.iter().filter_map(|bc| {
                let mut new_objective = objective.clone();
                for b in bc {
                    for bi in b {
                        if new_objective[*bi] == 0 {
                            return None;
                        }
                        new_objective[*bi] -= 1;
                    }
                }

                self.solve_part2_helper(patterns, &new_objective, cache)
                    .map(|r| r + bc.len())
            }));
        }

        if objective.iter().all(|i| *i % 2 == 0) {
            let new_objective = objective.iter().map(|i| i / 2).collect::<Vec<_>>();
            let res = self
                .solve_part2_helper(patterns, &new_objective, cache)
                .map(|r| r * 2);
            if let Some(r) = res {
                solutions.push(r);
            }
        }

        let res = solutions.iter().min().copied();
        cache.insert(objective.clone(), res);

        res
    }

    fn solve_part2(&self) -> usize {
        let patterns = self.compute_buttons_patterns();
        let mut cache = HashMap::new();

        self.solve_part2_helper(&patterns, &self.joltage, &mut cache)
            .unwrap()
    }
}

fn part1(machines: &[Machine]) -> usize {
    machines.iter().map(|m| m.solve_part1()).sum()
}

fn part2(machines: &[Machine]) -> usize {
    machines.iter().map(|m| m.solve_part2()).sum()
}

#[test]
fn test_parts() {
    let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    let machines = input
        .lines()
        .map(|l| l.parse::<Machine>().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(part1(&machines), 7);
    assert_eq!(part2(&machines), 33);
}

fn main() {
    let input = include_str!("../input");
    let machines = input
        .lines()
        .map(|l| l.parse::<Machine>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1 = {}", part1(&machines));
    println!("Part 2 = {}", part2(&machines));
}
