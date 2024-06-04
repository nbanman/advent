use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<HashSet<char>>> {
    input
        .split("\n\n")
        .map(|stanza| {
            stanza.lines()
                .map(|line| line.chars().collect())
                .collect()
        })
        .collect()
}

fn default_input() -> Vec<Vec<HashSet<char>>> {
    parse_input(include_input!(2020 / 06))
}

fn solve<F>(groups: Vec<Vec<HashSet<char>>>, f: F) -> usize
where
    F: Fn(HashSet<char>, HashSet<char>) -> HashSet<char>,
{
    groups
        .into_iter()
        .map(|group| group.into_iter().reduce(&f).unwrap().len())
        .sum()
}

fn part1(groups: Vec<Vec<HashSet<char>>>) -> usize {
    solve(groups, |acc, answer| acc.union(&answer).copied().collect())
}

fn part2(groups: Vec<Vec<HashSet<char>>>) -> usize {
    solve(groups, |acc, answer| {
        acc.intersection(&answer).copied().collect()
    })
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "abc

a
b
c

ab
ac

a
a
a
a

b",
    );
    assert_eq!(part1(input.clone()), 11);
    assert_eq!(part2(input), 6);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 6297);
    assert_eq!(part2(input), 3158);
}
