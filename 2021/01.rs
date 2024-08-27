use advent::prelude::*;

fn parse_input(input: &str) -> Vec<usize> {
    get_numbers(input)
}

fn count_increased<I>(measurements: I) -> usize
    where
        I: Iterator<Item=usize>,
{
    measurements
        .array_windows()
        .filter(|[a, b]| a < b)
        .count()
}

fn default_input() -> Vec<usize> {
    parse_input(include_input!(2021 / 01))
}

fn part1(measurements: Vec<usize>) -> usize {
    count_increased(measurements.into_iter())
}

fn part2(measurements: Vec<usize>) -> usize {
    let windows = measurements
        .windows(3)
        .map(|x| x.into_iter().sum());
    count_increased(windows)
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
    assert_eq!(part1(input.clone()), 1342);
    assert_eq!(part2(input), 1378);
}
