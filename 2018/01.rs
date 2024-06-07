use advent::prelude::*;

fn parse_input(input: &str) -> Vec<i64> {
    get_numbers(input)
}

fn default_input() -> Vec<i64> {
    parse_input(include_input!(2018 / 01))
}

fn part1(changes: Vec<i64>) -> i64 {
    changes.into_iter().sum()
}

fn part2(changes: Vec<i64>) -> i64 {
    let mut cache = HashSet::from_iter([0]);
    changes
        .into_iter()
        .cycle()
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .find(|&x| {
            !cache.insert(x)
        })
        .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    for (input, result) in [("+1 +1 +1", 3), ("+1 +1 -2", 0), ("-1 -2 -3", -6)] {
        assert_eq!(part1(parse_input(input)), result)
    }
}

#[test]
fn example2() {
    for (input, result) in [
        ("+1 -1", 0),
        ("+3 +3 +4 -2 -4", 10),
        ("-6 +3 +8 +5 -6", 5),
        ("+7 +7 -2 -7 -4", 14),
    ] {
        assert_eq!(part2(parse_input(input)), result)
    }
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 433);
    assert_eq!(part2(input), 256);
}
