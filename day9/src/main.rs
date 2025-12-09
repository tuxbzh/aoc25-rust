fn segments_intersect(
    s1: ((usize, usize), (usize, usize)),
    s2: ((usize, usize), (usize, usize)),
) -> bool {
    if s1.0 .0 == s1.1 .0 {
        // s1 is vertical
        return (s2.0 .1 > s1.0 .1.min(s1.1 .1))
            && (s2.0 .1 < s1.0 .1.max(s1.1 .1))
            && (s2.1 .1 > s1.0 .1.min(s1.1 .1))
            && (s2.1 .1 < s1.0 .1.max(s1.1 .1))
            && (s2.0 .0.min(s2.1 .0) < s1.0 .0)
            && (s2.0 .0.max(s2.1 .0) > s1.0 .0);
    }
    if s1.0 .1 == s1.1 .1 {
        // s1 is horizontal
        return (s2.0 .0 > s1.0 .0.min(s1.1 .0))
            && (s2.0 .0 < s1.0 .0.max(s1.1 .0))
            && (s2.1 .0 > s1.0 .0.min(s1.1 .0))
            && (s2.1 .0 < s1.0 .0.max(s1.1 .0))
            && (s2.0 .1.min(s2.1 .1) < s1.0 .1)
            && (s2.0 .1.max(s2.1 .1) > s1.0 .1);
    }
    panic!()
}

fn segment_intersect_puzzle(
    s: ((usize, usize), (usize, usize)),
    puzzle: &[(usize, usize)],
) -> bool {
    puzzle
        .windows(2)
        .any(|e| segments_intersect(s, (e[0], e[1])))
}

fn square_intersect_puzzle(
    a: (usize, usize),
    b: (usize, usize),
    puzzle: &[(usize, usize)],
) -> bool {
    let c = (a.0.min(b.0), a.1.min(b.1));
    let d = (a.0.max(b.0), a.1.max(b.1));
    let e = (a.0.max(b.0), a.1.min(b.1));
    let f = (a.0.min(b.0), a.1.max(b.1));

    segment_intersect_puzzle((c, e), puzzle)
        || segment_intersect_puzzle((e, d), puzzle)
        || segment_intersect_puzzle((d, f), puzzle)
        || segment_intersect_puzzle((f, c), puzzle)
        || puzzle.windows(2).any(|g| {
            // Find the case where a segment of the polygon exactly split the rectangle
            if g[0].0 == g[1].0 {
                // Vertical
                (g[0].0 > c.0)
                    && (g[0].0 < e.0)
                    && ((g[0].1 == c.1 && g[1].1 == f.1) || (g[1].1 == c.1 && g[0].1 == f.1))
            } else if g[0].1 == g[1].1 {
                // Horizontal
                (g[0].1 > c.1)
                    && (g[0].1 < f.1)
                    && ((g[0].0 == c.0 && g[1].0 == e.0) || (g[1].0 == c.0 && g[0].0 == e.0))
            } else {
                panic!()
            }
        })
}

fn puzzle_in_square(a: (usize, usize), b: (usize, usize), puzzle: &[(usize, usize)]) -> bool {
    let c = (a.0.min(b.0), a.1.min(b.1));
    let d = (a.0.max(b.0), a.1.max(b.1));

    puzzle
        .iter()
        .any(|p| (p.0 > c.0) && (p.0 < d.0) && (p.1 > c.1) && (p.1 < d.1))
}

fn part1(puzzle: &[(usize, usize)]) -> usize {
    let mut res = 0;
    for i in 0..(puzzle.len() - 1) {
        for j in (i + 1)..puzzle.len() {
            let a = puzzle[i];
            let b = puzzle[j];
            res = res.max((a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1));
        }
    }
    res
}

fn part2(puzzle: &[(usize, usize)]) -> usize {
    let mut res = 0;
    for i in 0..(puzzle.len() - 1) {
        for j in (i + 1)..puzzle.len() {
            let a = puzzle[i];
            let b = puzzle[j];

            if !square_intersect_puzzle(a, b, puzzle) && !puzzle_in_square(a, b, puzzle) {
                res = res.max((a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1));
                //println!("Max = {res}: {a:?} - {b:?}");
            }
        }
    }
    res
} // < 2445498515

#[test]
fn test_part2() {
    let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";
    let mut puzzle = input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();
    puzzle.push(*puzzle.first().unwrap());

    assert_eq!(part2(&puzzle), 24);
}

fn main() {
    let input = include_str!("../input");
    let mut puzzle = input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();
    puzzle.push(*puzzle.first().unwrap());

    println!("Part 1 = {}", part1(&puzzle));
    println!("Part 2 = {}", part2(&puzzle));
}
