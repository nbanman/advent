use std::ops::RangeInclusive;
use advent::prelude::*;

#[derive(Clone)]
struct PassPolicy<'a> {
    letter: u8,
    range: RangeInclusive<usize>,
    password: &'a [u8],
}

impl PassPolicy<'_> {
    fn is_valid_old(&self) -> bool {
        self.range.contains(&self.password.iter().filter(|&&c| c == self.letter).count())
    }

    fn is_valid_new(&self) -> bool {
        let start = self.password[self.range.start() - 1] == self.letter;
        let end = self.password[self.range.end() - 1] == self.letter;
        start ^ end
    }
}

fn parse_input(s: &str) -> Vec<PassPolicy<'_>> {
    s.lines()
        .map(|line| {
            let (lower, remainder) = line.split_once(['-', ' ']).unwrap();
            let (upper, remainder) = remainder.split_once(['-', ' ']).unwrap();
            let (letter, password) = remainder.split_once(['-', ' ']).unwrap();
            let lower = lower.parse().unwrap();
            let upper = upper.parse().unwrap();
            let range = lower..=upper;
            let letter = letter.as_bytes()[0];
            let password = password.as_bytes();
            PassPolicy { letter, range, password }
        })
        .collect()
}

fn default_input() -> Vec<PassPolicy<'static>> {
    parse_input(include_input!(2020 / 02))
}

fn part1(policies: Vec<PassPolicy<'_>>) -> usize {
    policies.into_iter()
        .filter(|policy| policy.is_valid_old())
        .count()
}

fn part2(policies: Vec<PassPolicy<'_>>) -> usize {
    policies.into_iter()
        .filter(|policy| policy.is_valid_new())
        .count()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc",
    );
    assert_eq!(part1(input.clone()), 2);
    assert_eq!(part2(input), 1);
}
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 445);
    assert_eq!(part2(input), 491);
}
