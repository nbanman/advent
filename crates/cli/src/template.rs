use advent::prelude::*;

fn parse_input(input: &str) -> Vec<i64> {
    todo!("parsing")
}

fn default_input() -> Vec<i64> {
    parse_input(include_str!("input/{day}.txt"))
}

fn part1(input: Vec<i64>) -> i64 {
    todo!("part 1")
}

fn part2(input: Vec<i64>) -> i64 {
    todo!("part 2")
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1);
    assert_eq!(part2(input), 2);
}
