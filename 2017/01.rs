use advent::prelude::*;

fn parse_input(input: &str) -> Vec<usize> {
    input.trim().as_bytes().iter().map(|b| (*b - 48) as usize).collect()
}

fn solve<F>(numbers: Vec<usize>, comparison_index: F) -> usize
    where
        F: Fn(usize, usize) -> usize,
{
    numbers.iter().enumerate()
        .filter(|(index, &i)| {
            numbers[comparison_index(*index, numbers.len()) % numbers.len()] == i
        })
        .map(|(_, i)| i)
        .sum()
}

fn default_input() -> Vec<usize> {
    parse_input(include_input!(2017 / 01))
}

fn part1(numbers: Vec<usize>) -> usize {
    solve(numbers, |index, _| index + 1)
}

fn part2(numbers: Vec<usize>) -> usize {
    solve(numbers, |index, size| index + size / 2)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1182);
    assert_eq!(part2(input), 1152);
}
