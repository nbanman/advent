use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(Command, i64)> {
    input
        .lines()
        .map(|line| {
            let (cmd, value) = line.split_once(char::is_whitespace).unwrap();
            (
                match cmd {
                    "forward" => Command::Forward,
                    "down" => Command::Down,
                    "up" => Command::Up,
                    d => panic!("unknown command `{d}`"),
                },
                value.parse().unwrap(),
            )
        })
        .collect()
}

fn default_input() -> Vec<(Command, i64)> {
    parse_input(include_input!(2021 / 02))
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Forward,
    Down,
    Up,
}

fn part1(input: Vec<(Command, i64)>) -> i64 {
    let (x, y) = input.iter().fold((0, 0), |(x, y), (cmd, v)| match cmd {
        Command::Forward => (x + v, y),
        Command::Down => (x, y + v),
        Command::Up => (x, y - v),
    });
    x * y
}

fn part2(input: Vec<(Command, i64)>) -> i64 {
    let (x, y, _) = input
        .iter()
        .fold((0, 0, 0), |(x, y, a), &(cmd, v)| match cmd {
            Command::Forward => (x + v, y + a * v, a),
            Command::Down => (x, y, a + v),
            Command::Up => (x, y, a - v),
        });
    x * y
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
forward 5
down 5
forward 8
up 3
down 8
forward 2",
    );
    assert_eq!(part1(input.clone()), 150);
    assert_eq!(part2(input), 900);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1654760);
    assert_eq!(part2(input), 1956047400);
}
