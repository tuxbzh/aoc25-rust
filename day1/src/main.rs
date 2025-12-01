fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.split_at(1))
        .map(|(d, i)| (d, i.parse::<isize>().unwrap()))
        .fold((50isize, 0usize), |(pos, count), (dir, disp)| {
            let new_pos = (if dir == "L" { pos - disp } else { pos + disp }).rem_euclid(100);
            let new_count = if new_pos == 0 { count + 1 } else { count };
            (new_pos, new_count)
        })
        .1
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.split_at(1))
        .map(|(d, i)| (d, i.parse::<isize>().unwrap()))
        .fold((50isize, 0usize), |(pos, count), (dir, disp)| {
            assert!(pos >= 0);
            assert!(pos < 100);
            let disp_d = disp.div_euclid(100);
            let disp_r = disp.rem_euclid(100);
            let mut new_count = count + disp_d.unsigned_abs();
            let new_pos = if dir == "L" {
                pos - disp_r
            } else {
                pos + disp_r
            };
            if pos != 0 && (new_pos <= 0 || new_pos >= 100) {
                new_count += 1;
            }
            (new_pos.rem_euclid(100), new_count)
        })
        .1
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}
