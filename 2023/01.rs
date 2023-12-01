use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 01)
}

fn part1(calibration: &str) -> i64 {
    3
}

fn part2(calibration: &str) -> i64 {
    3
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
