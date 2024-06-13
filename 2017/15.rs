use advent::prelude::*;

const FACTOR_A: usize = 16807;
const FACTOR_B: usize = 48271;


fn parse_input(input: &str) -> (usize, usize) {
    let seeds: Vec<usize> = input
        .lines()
        .map(|line| {
            line.split(' ').last().unwrap().parse().unwrap()
        })
        .collect();
    (*seeds.first().unwrap(), *seeds.last().unwrap())
}

fn default_input() -> (usize, usize) {
    parse_input(include_input!(2017 / 15))
}

fn generator(seed: usize, factor: usize, multiples: Option<usize>) -> impl Iterator<Item=u16> {
    iter::successors(Some(seed), move |&prev| Some((prev * factor) % 2147483647))
        .filter(move |&value| {
            if let Some(multiples) = multiples { value % multiples == 0 } else { true }
        })
        .map(|value| value as u16)
        .skip(1)
}

fn solve(
    comparisons: usize,
    generator_a: impl Iterator<Item=u16>,
    generator_b: impl Iterator<Item=u16>,
) -> usize {
    generator_a.zip(generator_b)
        .take(comparisons)
        .filter(|(a, b)| a == b)
        .count()
}

fn part1((seed_a, seed_b): (usize, usize)) -> usize {
    solve(
        40_000_000,
        generator(seed_a, FACTOR_A, None),
        generator(seed_b, FACTOR_B, None),
    )
}

fn part2((seed_a, seed_b): (usize, usize)) -> usize {
    solve(
        5_000_000,
        generator(seed_a, FACTOR_A, Some(4)),
        generator(seed_b, FACTOR_B, Some(8)),
    )
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 594);
    assert_eq!(part2(input), 328);
}
