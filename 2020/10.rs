use std::ops::Mul;
use advent::prelude::*;

fn parse_input(input: &str) -> Vec<usize> {
    let mut joltage_differences: VecDeque<_> = input
        .get_numbers()
        .sorted()
        .collect();
    joltage_differences.push_front(0);
    joltage_differences.push_back(joltage_differences.iter().last().unwrap() + 3);
    joltage_differences
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect()
}

fn default_input() -> Vec<usize> {
    parse_input(include_input!(2020 / 10))
}

fn part1(joltage_differences: Vec<usize>) -> usize {
    let jolt3s = joltage_differences
        .iter()
        .filter(|&jolt_diff| jolt_diff == &3)
        .count();
    let jolt1s = joltage_differences.len() - jolt3s;
    jolt1s * jolt3s
}

fn part2(joltage_differences: Vec<usize>) -> usize {
    joltage_differences
        .split(|jolt_diff| jolt_diff == &3)
        .map(|sublist| {
            match sublist.len() {
                4 => 7,
                3 => 4,
                2 => 2,
                _ => 1,
            }
        })
        .fold(1, usize::mul)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input("16 10 15 5 1 11 7 19 6 12 4");
    assert_eq!(part1(input.clone()), 35);
    assert_eq!(part2(input), 8);
}

#[test]
fn example2() {
    let input = parse_input(
        "28 33 18 42 31 14 46 20 48 47 24 23 49 45 \
         19 38 39 11 1 32 25 35 8 17 7 9 4 2 34 10 3",
    );
    assert_eq!(part1(input.clone()), 220);
    assert_eq!(part2(input), 19208);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1890);
    assert_eq!(part2(input), 49607173328384);
}
