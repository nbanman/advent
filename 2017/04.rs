use std::hash::Hash;

use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input.lines().map(|line| line.split(|c| c == ' ').collect()).collect()
}

fn default_input() -> Vec<Vec<&'static str>> {
    parse_input(include_input!(2017 / 04))
}

fn count_unique<T>(passphrases: Vec<Vec<T>>) -> usize
    where
        T: Eq + Hash,
{
    passphrases.into_iter()
        .filter(|phrase| {
            let distinct = HashSet::from_iter(phrase.iter());
            phrase.len() == distinct.len()
        }).count()
}

fn part1(passphrases: Vec<Vec<&str>>) -> usize {
    count_unique(passphrases)
}

fn part2(passphrases: Vec<Vec<&str>>) -> usize {
    let distributions = passphrases.into_iter()
        .map(|phrase| {
            phrase.iter()
                .map(|&word| {
                    let mut distribution: HashMap<char, usize> = HashMap::new();
                    for c in word.chars() {
                        *distribution.entry(c).or_default() += 1;
                    }
                    distribution.into_iter().sorted_by_key(|(k, _)| k.to_owned()).collect::<Vec<_>>()
                })
                .collect()
        })
        .collect();
    count_unique(distributions)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 455);
    assert_eq!(part2(input), 186);
}
