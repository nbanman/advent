use advent::prelude::*;

fn default_input() -> Vec<u8> {
    parse_input(include_input!(2018 / 05).trim())
}

fn parse_input(input: &str) -> Vec<u8> {
    react(input.as_bytes().iter(), None)
}

fn react<'a>(polymer: impl Iterator<Item=&'a u8>, removed: Option<u8>) -> Vec<u8> {
    let mut stack: Vec<u8> = Vec::new();
    for a in polymer {
        if removed != None && (*a == removed.unwrap() || *a == removed.unwrap().to_ascii_uppercase()) {
            // nada
        } else {
            if stack.is_empty() || (*stack.last().unwrap() as isize - *a as isize).abs() != 32 {
                stack.push(a.to_owned())
            } else {
                stack.pop();
            }
        }
    }
    stack
}

fn part1(polymer: Vec<u8>) -> usize {
    polymer.len()
}

fn part2(polymer: Vec<u8>) -> usize {
    (b'a'..=b'z')
        .map(|removed| react(polymer.iter(), Some(removed)).len())
        .min()
        .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = "dabAcCaCBAcCcaDA";
    assert_eq!(part1(parse_input(input)), 10);
    assert_eq!(part2(parse_input(input)), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 10972);
    assert_eq!(part2(input), 5278);
}
