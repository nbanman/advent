use advent::prelude::*;

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| {
            line.as_bytes().iter().enumerate()
                .fold(0, |acc, (index, c)| {
                    acc + if c == &b'B' || c == &b'R' {
                        1 << (9 - index)
                    } else {
                        0
                    }
                })
        })
        .sorted()
        .collect()
}

fn default_input() -> Vec<u64> {
    parse_input(include_input!(2020 / 05))
}

fn part1(ids: Vec<u64>) -> u64 {
    *ids.last().unwrap()
}

fn part2(ids: Vec<u64>) -> u64 {
    ids.iter()
        .tuple_windows()
        .find(|(&prev, &next)| prev + 1 != next)
        .unwrap()
        .0 + 1
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL",
    );
    assert_eq!(part1(input), 820);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 922);
    assert_eq!(part2(input), 747);
}
