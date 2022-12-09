use advent::prelude::*;

fn parse_input(s: &str) -> Vec<Element<'_>> {
    regex!(r"(?P<lower>\d+)-(?P<upper>\d+)\s(?P<letter>.):\s(?P<password>.*)")
        .captures_iter(s)
        .map(|caps| Element {
            lower: caps["lower"].parse().unwrap(),
            upper: caps["upper"].parse().unwrap(),
            letter: caps["letter"].parse().unwrap(),
            password: caps.name("password").unwrap().as_str(),
        })
        .collect()
}

fn default_input() -> Vec<Element<'static>> {
    parse_input(include_str!("input/02.txt"))
}

#[derive(Debug, Clone)]
struct Element<'a> {
    lower: usize,
    upper: usize,
    letter: char,
    password: &'a str,
}

fn part1(elements: Vec<Element<'_>>) -> usize {
    elements
        .into_iter()
        .filter(|e| {
            let freq = e.password.chars().filter(|&c| c == e.letter).count();
            (e.lower..=e.upper).contains(&freq)
        })
        .count()
}

fn part2(elements: Vec<Element<'_>>) -> usize {
    elements
        .into_iter()
        .filter(|e| {
            (e.password.chars().nth(e.lower - 1).unwrap() == e.letter)
                ^ (e.password.chars().nth(e.upper - 1).unwrap() == e.letter)
        })
        .count()
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc",
    );
    assert_eq!(part1(input.clone()), 2);
    assert_eq!(part2(input), 1);
}
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 383);
    assert_eq!(part2(input), 272);
}
