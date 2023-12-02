use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 01)
}

fn calibrate(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find(|c| c.is_digit(10))
                .unwrap()
                .to_digit(10)
                .unwrap();
            let second = line
                .chars()
                .rfind(|c| c.is_digit(10))
                .unwrap()
                .to_digit(10)
                .unwrap();
            first * 10 + second
        })
        .sum()
}

fn part1(input: &str) -> u32 {
    calibrate(input)
}

fn part2(input: &str) -> u32 {
    let replacements = [
        ["one", "o1e"],
        ["two", "t2o"],
        ["three", "t3e"],
        ["four", "4"],
        ["five", "5e"],
        ["six", "6"],
        ["seven", "7n"],
        ["eight", "e8t"],
        ["nine", "n9e"],
    ];

    let replaced_input = replacements
        .iter()
        .fold(input.to_string(), |acc, [original, replacement]| {
            acc.replace(original, &replacement)
        });

    calibrate(&replaced_input)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    assert_eq!(part1(input), 142);
    assert_eq!(part2(input), 142);
}

#[test]
fn example2() {
    let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_eq!(part2(input), 281);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 54388);
    assert_eq!(part2(input), 53515);
}
