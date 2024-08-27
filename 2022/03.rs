use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.bytes().map(priority).collect())
        .collect()
}

fn default_input() -> Vec<Vec<u32>> {
    parse_input(include_input!(2022 / 03))
}

fn priority(item: u8) -> u32 {
    let p = match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
    };
    p as u32
}

fn bitset(chunk: &[u32]) -> u64 {
    chunk.iter().fold(0, |acc, p| { acc | (1u64 << p) })
}

fn count_trailing_zero_bits(l: u64) -> u32 {
    let mut zero_bits = 0;
    let mut v = l;
    while v != 0 {
        if v & 1 == 0 { zero_bits += 1 };
        v /= 2;
    }
    zero_bits
}

fn part1(rucksacks: Vec<Vec<u32>>) -> u32 {
    rucksacks
        .iter()
        .map(|rucksack| {
            let m = rucksack.len() / 2;
            let a = bitset(&rucksack[..m]);
            let b = bitset(&rucksack[m..]);
            count_trailing_zero_bits(a & b)
        }).sum()
}

fn part2(rucksacks: Vec<Vec<u32>>) -> u32 {
    rucksacks
        .iter()
        .map(|rucksack| bitset(rucksack))
        .array_chunked::<3>()
        .map(|tres| {
            let x = tres.into_iter().reduce(|acc, bitset| { acc & bitset }).unwrap();
            count_trailing_zero_bits(x)
        }).sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
    );
    assert_eq!(part1(input.clone()), 157);
    assert_eq!(part2(input), 70);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 7428);
    assert_eq!(part2(input), 2650);
}
