use std::str::Lines;
use advent::prelude::*;
use once_cell::sync::Lazy;
use regex::Regex;

fn parse_input(input: &'static str) -> (Lines<'static>, usize) {
    (input.lines(), input.lines().map(|line| line.len()).sum())
}

fn default_input() -> (Lines<'static>, usize) {
    parse_input(include_input!(2015 / 08))
}

fn part1((lines, total_length): (Lines<'static>, usize)) -> usize {
    total_length - lines.map(|line| {
        let cim = chars_in_memory(line);
        // println!("{}: {}", cim, line);
        cim
    }).sum::<usize>()
}

static REPLACE_RX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\\\\|\\"|\\x[\da-f]{2}"#).unwrap()
});
fn chars_in_memory(line: &str) -> usize {
    REPLACE_RX.replace_all(&line[1..line.len() - 1], "X").len()
}

fn part2((lines, total_length): (Lines<'static>, usize)) -> usize {
    lines.into_iter().map(|line| encoded_length(line)).sum::<usize>() - total_length
}

fn encoded_length(line: &str) -> usize {
    line.len() + 2 + line.chars().filter(|&c| c == '\\' || c == '"').count()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1333);
    assert_eq!(part2(input), 2046);
}
