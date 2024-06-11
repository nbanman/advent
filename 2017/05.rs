use advent::prelude::*;

fn parse_input(input: &str) -> Vec<i64> { get_numbers(input) }

fn default_input() -> Vec<i64> {
    parse_input(include_input!(2017 / 05))
}

fn solve<F>(mut offsets: Vec<i64>, increment: F) -> i64
where
    F: Fn(i64) -> i64,
{
    let mut i = 0i64;
    let mut steps = 0;
    let length = offsets.len() as i64;
    while i < length {
        steps += 1;
        let temp = offsets[i as usize];
        offsets[i as usize] += increment(offsets[i as usize]);
        i += temp;
    }
    steps
}
fn part1(jumps: Vec<i64>) -> i64 {
    solve(jumps, |_| 1)
}

fn part2(jumps: Vec<i64>) -> i64 {
    solve(jumps, |offset| if offset >= 3 { -1 } else { 1 })
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 373160);
    assert_eq!(part2(input), 26395586);
}
