use advent::prelude::*;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn default_input() -> Vec<i64> {
    parse_input(include_input!(2021 / 01))
}

fn part1(input: Vec<i64>) -> usize {
    input
        .into_iter()
        .array_windows()
        .filter(|[a, b]| b > a)
        .count()
}

fn part2(input: Vec<i64>) -> usize {
    input
        .into_iter()
        .array_windows()
        .filter(|[a, _, _, b]| b > a)
        .count()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input("199 200 208 210 200 207 240 269 260 263");
    assert_eq!(part1(input.clone()), 7);
    assert_eq!(part2(input), 5);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1448);
    assert_eq!(part2(input), 1471);
}
