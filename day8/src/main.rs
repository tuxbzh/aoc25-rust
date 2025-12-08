use std::{collections::HashSet, convert::Infallible, str::FromStr};

struct Puzzle {
    pos: Vec<(usize, usize, usize)>,
    dis: Vec<((usize, usize, usize), (usize, usize, usize), usize)>,
}

impl FromStr for Puzzle {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos = s
            .lines()
            .map(|l| {
                let mut sp = l.split(',');
                let x = sp.next().unwrap().parse::<usize>().unwrap();
                let y = sp.next().unwrap().parse::<usize>().unwrap();
                let z = sp.next().unwrap().parse::<usize>().unwrap();
                (x, y, z)
            })
            .collect::<Vec<_>>();

        let mut dis = Vec::new();
        for i in 0..pos.len() {
            for j in (i + 1)..pos.len() {
                let a = *pos.get(i).unwrap();
                let b = *pos.get(j).unwrap();
                let dx = a.0.abs_diff(b.0);
                let dy = a.1.abs_diff(b.1);
                let dz = a.2.abs_diff(b.2);
                let dist = dx * dx + dy * dy + dz * dz;
                dis.push((a, b, dist));
            }
        }

        dis.sort_unstable_by_key(|(_, _, d)| *d);

        Ok(Puzzle { pos, dis })
    }
}

fn part1(puzzle: &Puzzle, iter: usize) -> usize {
    let mut circuits: Vec<HashSet<(usize, usize, usize)>> = Vec::new();

    //println!("{:?}", puzzle.dis);

    for i in 0..iter {
        let (a, b, _) = puzzle.dis.get(i).unwrap();

        //println!("{:?} - {:?}", *a, *b);

        let a_circ = circuits
            .iter()
            .enumerate()
            .find(|(_, circuit)| circuit.contains(a))
            .map(|(idx, _)| idx);

        let b_circ = circuits
            .iter()
            .enumerate()
            .find(|(_, circuit)| circuit.contains(b))
            .map(|(idx, _)| idx);

        //println!("  {:?} - {:?}", a_circ, b_circ);

        match (a_circ, b_circ) {
            (None, None) => {
                let mut circ = HashSet::new();
                circ.insert(*a);
                circ.insert(*b);
                circuits.push(circ);
            }
            (Some(aidx), None) => {
                circuits.get_mut(aidx).unwrap().insert(*b);
            }
            (None, Some(bidx)) => {
                circuits.get_mut(bidx).unwrap().insert(*a);
            }
            (Some(aidx), Some(bidx)) => {
                if aidx != bidx {
                    let bc = circuits.get(bidx).unwrap().clone();
                    circuits.get_mut(aidx).unwrap().extend(bc);
                    circuits.remove(bidx);
                }
            }
        }
    }

    circuits.sort_unstable_by_key(|c| -(c.len() as isize));

    //println!("{circuits:?}");

    circuits.first().unwrap().len()
        * circuits.get(1).unwrap().len()
        * circuits.get(2).unwrap().len()
}

fn part2(puzzle: &Puzzle) -> usize {
    let mut circuits: Vec<HashSet<(usize, usize, usize)>> = Vec::new();

    //println!("{:?}", puzzle.dis);

    for i in 0..puzzle.dis.len() {
        let (a, b, _) = puzzle.dis.get(i).unwrap();

        //println!("{:?} - {:?}", *a, *b);

        let a_circ = circuits
            .iter()
            .enumerate()
            .find(|(_, circuit)| circuit.contains(a))
            .map(|(idx, _)| idx);

        let b_circ = circuits
            .iter()
            .enumerate()
            .find(|(_, circuit)| circuit.contains(b))
            .map(|(idx, _)| idx);

        //println!("  {:?} - {:?}", a_circ, b_circ);

        match (a_circ, b_circ) {
            (None, None) => {
                let mut circ = HashSet::new();
                circ.insert(*a);
                circ.insert(*b);
                circuits.push(circ);
            }
            (Some(aidx), None) => {
                circuits.get_mut(aidx).unwrap().insert(*b);
            }
            (None, Some(bidx)) => {
                circuits.get_mut(bidx).unwrap().insert(*a);
            }
            (Some(aidx), Some(bidx)) => {
                if aidx != bidx {
                    let bc = circuits.get(bidx).unwrap().clone();
                    circuits.get_mut(aidx).unwrap().extend(bc);
                    circuits.remove(bidx);
                }
            }
        }

        if circuits.len() == 1 && circuits.first().unwrap().len() == puzzle.pos.len() {
            return a.0 * b.0;
        }
    }

    unreachable!()
}

#[test]
fn test_part1() {
    let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";
    let puzzle = input.parse::<Puzzle>().unwrap();
    assert_eq!(part1(&puzzle, 10), 40);
}

fn main() {
    let input = include_str!("../input");
    let puzzle = input.parse::<Puzzle>().unwrap();
    println!("Part 1 = {}", part1(&puzzle, 1000));
    println!("Part 2 = {}", part2(&puzzle));
}
