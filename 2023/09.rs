use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(|line| {
        line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>()
    }).collect()
}

fn default_input() -> Vec<Vec<i64>> {
    parse_input(include_input!(2023 / 09))
}

// stole algo from @ephemient
fn extrapolate(pattern: &Vec<i64>) -> i64 {
    let mut c = 1;
    let mut s = 0;
    for (i, x) in pattern.iter().enumerate() {
        s = c * x - s;
        c = c * (pattern.len() - i) as i64 / (i + 1) as i64;
    }
    let answer = s;
    answer
}
fn part1(patterns: Vec<Vec<i64>>) -> i64 {
    patterns.iter().map(|pattern| extrapolate(pattern)).sum()
}

fn part2(patterns: Vec<Vec<i64>>) -> i64 {
    patterns.iter().map(|pattern| {
        let reversed: Vec<i64> = pattern.iter().rev().cloned().collect();
        extrapolate(&reversed)
    }).sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1974913025);
    assert_eq!(part2(input), 884);
}
