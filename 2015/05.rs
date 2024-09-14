use advent::prelude::*;
use fancy_regex::Regex;

fn default_input() -> &'static str {
    include_input!(2015 / 05)
}

fn part1(input: &'static str) -> usize {
    let vowels = regex!(r"[aeiou]");
    let at_least_3_vowels = |s: &str| vowels.find_iter(s).count() >= 3;
    let repeated_letter = Regex::new(r"([a-z])\1").unwrap();
    let at_least_1_twice = |s: &str| repeated_letter.is_match(s).unwrap();
    let forbidden = regex!(r"ab|cd|pq|xy");
    let no_forbidden_strings = |s: &str| !forbidden.is_match(s);

    input
        .lines()
        .filter(|&s| {
            at_least_3_vowels(s)
                && at_least_1_twice(s)
                && no_forbidden_strings(s)
        })
        .count()
}

fn part2(input: &'static str) -> usize {
    let repeated_duo = Regex::new(r"([a-z]{2}).*\1").unwrap();
    let at_least_2_twice = |s: &str| repeated_duo.is_match(s).unwrap();
    let repeated_1_between = Regex::new(r"([a-z]).\1").unwrap();
    let repeats_with_1_between = |s: &str| repeated_1_between.is_match(s).unwrap();

    input.lines().filter(|&s| at_least_2_twice(s) && repeats_with_1_between(s)).count()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 255);
    assert_eq!(part2(input), 55);
}
