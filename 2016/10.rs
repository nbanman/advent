use std::cmp::PartialEq;
use std::hash::Hash;
use std::ops::Mul;
use itertools::MinMaxResult::MinMax;
use regex::Regex;
use advent::prelude::*;
use crate::Destination::{Bot, Output};

fn default_input() -> &'static str {
    include_input!(2016 / 10)
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
enum Destination {
    Bot,
    Output,
}

fn get_destination(s: &str) -> Destination {
    match s {
        "bot" => Bot,
        _ => Output,
    }
}

fn solve(input: &'static str, part2: bool) -> usize {
    let pattern= r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)";
    let bots: HashMap<usize, ((Destination, usize), (Destination, usize))> = Regex::new(pattern)
        .unwrap()
        .captures_iter(input)
        .map(|captures| {
            let id: usize = captures.get(1).unwrap().as_str().parse().unwrap();
            let low_dest = get_destination(captures.get(2).unwrap().as_str());
            let low: usize = captures.get(3).unwrap().as_str().parse().unwrap();
            let high_dest = get_destination(captures.get(4).unwrap().as_str());
            let high: usize = captures.get(5).unwrap().as_str().parse().unwrap();
            (id, ((low_dest, low), (high_dest, high)))
        })
        .collect();
    println!("Bots: {:?}", bots.len());
    let mut registry = HashMap::new();
    let pattern = r"value (\d+) goes to bot (\d+)";
    for captures in Regex::new(pattern).unwrap().captures_iter(input) {
        let value = captures.get(1).unwrap().as_str().parse().unwrap();
        let bot = captures.get(2).unwrap().as_str().parse().unwrap();
        if let Some(responsible) = assign(
            (Destination::Bot, bot),
            value,
            &bots,
            &mut registry,
            part2,
        ) {
            if !part2 { return responsible }
        }
    }
    println!("Registry: {:?}", registry.len());
    registry
        .into_iter()
        .filter(|((dest, value), _)| *dest == Output && *value <= 2)
        .map(|(_, value)| value)
        .reduce(usize::mul)
        .unwrap()
}

fn assign(
    recipient: (Destination, usize),
    value: usize,
    bots: &HashMap<usize, ((Destination, usize), (Destination, usize))>,
    registry: &mut HashMap<(Destination, usize), usize>,
    part2: bool,
) -> Option<usize> {

    if let Some(&current) = registry.get(&recipient) {
        if let MinMax(low, high) = [current, value].into_iter().minmax() {
            if !part2 && low == 17 && high == 61 { return Some(recipient.1); }

            let (low_recipient, high_recipient) = bots.get(&recipient.1).unwrap();
            if low_recipient.0 == Output && low_recipient.1 <= 2 {
                if let Some(responsible) = assign(
                    *low_recipient,
                    low,
                    bots,
                    registry,
                    part2,
                ) {
                    if !part2 { return Some(responsible); }
                };
            };
            if high_recipient.0 == Output && high_recipient.1 <= 2 {
                if let Some(responsible) = assign(
                    *high_recipient,
                    high,
                    bots,
                    registry,
                    part2,
                ) {
                    if !part2 { return Some(responsible); }
                };
            };
            registry.remove(&recipient);
        }
    } else {
        registry.insert(recipient, value);
    }
    None
}

fn part1(input: &'static str) -> usize {
    solve(input, false)
}

fn part2(input: &'static str) -> usize {
    solve(input, true)
}
fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 101);
    assert_eq!(part2(input), 37789);
}
