use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 01)
}
fn part1(input: &str) -> i64 {
    input.as_bytes().iter()
        .map(|&c| if c == b'(' { 1 } else { -1 })
        .sum()
}

fn part2(input: &str) -> usize {
    input.as_bytes().iter()
        .map(|&c| if c == b'(' { 1 } else { -1 })
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .enumerate()
        .find(|(_, x)| x == &-1)
        .unwrap()
        .0 + 1
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 280);
    assert_eq!(part2(input), 1797);
}
