use fancy_regex::Regex;

fn part1(input: &str) -> usize {
    let re = Regex::new(r"^(\d+)(\1)$").unwrap();

    input
        .trim_end()
        .split(',')
        .map(|s| s.split_once('-').unwrap())
        .map(|(s, e)| (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap()))
        .flat_map(|(s, e)| s..=e)
        .filter(|i| re.is_match(&i.to_string()).unwrap())
        .sum()
}

fn part2(input: &str) -> usize {
    let re = Regex::new(r"^(\d+)(\1)+$").unwrap();

    input
        .trim_end()
        .split(',')
        .map(|s| s.split_once('-').unwrap())
        .map(|(s, e)| (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap()))
        .flat_map(|(s, e)| s..=e)
        .filter(|i| re.is_match(&i.to_string()).unwrap())
        .sum()
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}
