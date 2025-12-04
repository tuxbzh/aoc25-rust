use pathfinding::matrix::*;

fn find_removable(input: &Matrix<char>) -> Vec<(usize, usize)> {
    input
        .items()
        .filter(|(_, c)| **c == '@')
        .filter(|(p, _)| {
            input
                .neighbours(*p, true)
                .filter(|np| *input.get(*np).unwrap() == '@')
                .count()
                < 4
        })
        .map(|(p, _)| p)
        .collect::<Vec<_>>()
}

fn part1(input: &Matrix<char>) -> usize {
    find_removable(input).len()
}

fn part2(mut input: Matrix<char>) -> usize {
    let mut res = 0;
    loop {
        let rem = find_removable(&input);
        if rem.is_empty() {
            return res;
        }
        res += rem.len();
        rem.iter().for_each(|p| *input.get_mut(*p).unwrap() = 'x');
    }
}

fn main() {
    let input = include_str!("../input");
    let m = Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();
    println!("Part 1 = {}", part1(&m));
    println!("Part 2 = {}", part2(m));
}
