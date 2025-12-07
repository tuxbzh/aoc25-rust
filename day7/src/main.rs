use std::collections::{HashMap, HashSet};

use pathfinding::matrix::Matrix;

fn part1(puzzle: &Matrix<char>) -> usize {
    let (start, _) = puzzle.items().find(|(_, c)| **c == 'S').unwrap();
    let mut pos_to_visit = Vec::new();
    let mut splitter_visited = HashSet::new();
    let mut pos_visited = HashSet::new();
    let mut current_pos = start;

    loop {
        if !pos_visited.insert(current_pos) {
            if pos_to_visit.is_empty() {
                break;
            }
            current_pos = pos_to_visit.pop().unwrap();
            continue;
        }

        if let Some(new_pos) = puzzle.move_in_direction(current_pos, (1, 0)) {
            if *puzzle.get(new_pos).unwrap() == '^' {
                splitter_visited.insert(new_pos);

                let left = puzzle.move_in_direction(new_pos, (0, -1)).unwrap();
                let right = puzzle.move_in_direction(new_pos, (0, 1)).unwrap();

                pos_to_visit.push(right);
                current_pos = left;
            } else {
                current_pos = new_pos;
            }
        } else {
            if pos_to_visit.is_empty() {
                break;
            }
            current_pos = pos_to_visit.pop().unwrap();
        }
    }

    splitter_visited.len()
}

fn num_path_from_splitter(
    puzzle: &Matrix<char>,
    splitter_pos: (usize, usize),
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(num) = cache.get(&splitter_pos) {
        return *num;
    }

    let mut res;

    let left = puzzle.move_in_direction(splitter_pos, (0, -1)).unwrap();
    let right = puzzle.move_in_direction(splitter_pos, (0, 1)).unwrap();

    let mut current_pos = left;

    // Left
    loop {
        if let Some(new_pos) = puzzle.move_in_direction(current_pos, (1, 0)) {
            if *puzzle.get(new_pos).unwrap() == '^' {
                res = num_path_from_splitter(puzzle, new_pos, cache);
                break;
            } else {
                current_pos = new_pos;
            }
        } else {
            res = 1;
            break;
        }
    }

    current_pos = right;

    // Right
    loop {
        if let Some(new_pos) = puzzle.move_in_direction(current_pos, (1, 0)) {
            if *puzzle.get(new_pos).unwrap() == '^' {
                res += num_path_from_splitter(puzzle, new_pos, cache);
                break;
            } else {
                current_pos = new_pos;
            }
        } else {
            res += 1;
            break;
        }
    }

    cache.insert(splitter_pos, res);
    res
}

fn part2(puzzle: &Matrix<char>) -> usize {
    let (start, _) = puzzle.items().find(|(_, c)| **c == 'S').unwrap();
    let mut cache = HashMap::new();
    let mut current_pos = start;

    loop {
        if let Some(new_pos) = puzzle.move_in_direction(current_pos, (1, 0)) {
            if *puzzle.get(new_pos).unwrap() == '^' {
                return num_path_from_splitter(puzzle, new_pos, &mut cache);
            } else {
                current_pos = new_pos;
            }
        }
    }
}

fn main() {
    let input = include_str!("../input");
    let puzzle = Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();
    println!("Part 1 = {}", part1(&puzzle));
    println!("Part 2 = {}", part2(&puzzle));
}
