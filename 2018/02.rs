extern crate core;

use core::str;
use std::str::SplitWhitespace;
use advent::prelude::*;

fn parse_input(input: &str) -> SplitWhitespace<'_> {
    input.split_whitespace()
}

fn default_input() -> SplitWhitespace<'static> {
    parse_input(include_input!(2018 / 02))
}

fn part1(box_ids: SplitWhitespace<'_>) -> usize {
    let frequencies = box_ids
        .map(|box_id| {
            box_id.as_bytes().iter()
                .fold(HashMap::new(), |mut acc, &item| {
                    *acc.entry(item).or_insert(0) += 1;
                    acc
                })
        });

    let twos = frequencies
        .clone()
        .filter(|it| it.values().contains(&2))
        .count();

    let threes = frequencies
        .filter(|it| it.values().contains(&3))
        .count();
    twos * threes
}

fn part2(box_ids: SplitWhitespace<'_>) -> String {
    fn differs_by_one<'a>(a: &'a str, b: &'a str) -> bool {
        let mut diffs = false;
        let a = a.as_bytes();
        let b = b.as_bytes();
        for (idx, c) in a.into_iter().enumerate() {
            if c != &b[idx] {
                if !diffs {
                    diffs = true
                } else {
                    return false
                }
            }
        }
        return diffs
    }
    let box_ids: Vec<_> = box_ids.collect();
    for (&a, &b) in box_ids[..box_ids.len() - 1].iter().cartesian_product(box_ids[1..].iter()) {
        if differs_by_one(a, b) {
            let x = a.as_bytes().into_iter().zip(b.as_bytes().into_iter());
            let x: Vec<_> = x
                .filter_map(|(a, b)| if a == b { Some(*a) } else { None })
                .collect();
            return String::from_utf8(x).unwrap()
        }
    }
    unreachable!()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        "\
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab",
    );
    assert_eq!(part1(input), 12)
}

#[test]
fn example2() {
    let input = parse_input(
        "\
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz",
    );
    assert_eq!(part2(input), "fgij")
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 7688);
    assert_eq!(part2(input), "lsrivmotzbdxpkxnaqmuwcchj");
}
