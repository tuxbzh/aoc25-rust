fn process_line(l: &str) -> u32 {
    let d1 = l[0..l.len() - 1].chars().max().unwrap();
    let pos_d1 = l.chars().enumerate().find(|(_, c)| *c == d1).unwrap().0;
    let d2 = l[pos_d1 + 1..].chars().max().unwrap();
    d1.to_digit(10).unwrap() * 10 + d2.to_digit(10).unwrap()
}

#[test]
fn test_process_line() {
    assert_eq!(process_line("987654321111111"), 98);
    assert_eq!(process_line("811111111111119"), 89);
    assert_eq!(process_line("234234234234278"), 78);
    assert_eq!(process_line("818181911112111"), 92);
}

fn process_line_part2(l: &str) -> usize {
    let mut digits = String::new();

    let d = l[0..l.len() - 11].chars().max().unwrap();
    let mut pos_d = l.chars().enumerate().find(|(_, c)| *c == d).unwrap().0;
    digits.push(d);

    for i in 1..12 {
        let next_d = l[pos_d + 1..(l.len() - 11 + i)].chars().max().unwrap();
        let next_pos_d = l[pos_d + 1..(l.len() - 11 + i)]
            .chars()
            .enumerate()
            .find(|(_, c)| *c == next_d)
            .unwrap()
            .0
            + pos_d
            + 1;
        digits.push(next_d);

        pos_d = next_pos_d;
    }

    digits.parse::<usize>().unwrap()
}

#[test]
fn test_process_line_part2() {
    assert_eq!(process_line_part2("987654321111111"), 987654321111);
    assert_eq!(process_line_part2("811111111111119"), 811111111119);
    assert_eq!(process_line_part2("234234234234278"), 434234234278);
    assert_eq!(process_line_part2("818181911112111"), 888911112111);
}

fn part1(input: &str) -> u32 {
    input.lines().map(process_line).sum()
}

fn part2(input: &str) -> usize {
    input.lines().map(process_line_part2).sum()
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}
