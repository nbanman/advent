use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(&str, isize)> {
    input.lines()
        .map(|line| {
            let (command, amt) = line.split_once(' ').unwrap();
            (command, amt.parse().unwrap())
        })
        .collect()
}

fn default_input() -> Vec<(&'static str, isize)> {
    parse_input(include_input!(2021 / 02))
}

fn solve<F>(commands: Vec<(&str, isize)>, interpretation: F) -> isize
    where
        F: Fn((isize, isize, isize), (&str, isize)) -> (isize, isize, isize)
{
    let (x, y, _) = commands.into_iter().fold((0isize, 0isize, 0isize), interpretation);
    x * y
}

fn part1(commands: Vec<(&str, isize)>) -> isize {
    solve(commands, |(x, y, _), (cmd, amt)| {
        match cmd {
            "forward" => (x + amt, y, 0),
            "down" => (x, y + amt, 0),
            "up" => (x, y - amt, 0),
            _ => (x, y, 0),
        }
    })
}

fn part2(commands: Vec<(&str, isize)>) -> isize {
    solve(commands, |(x, y, aim), (cmd, amt)| {
        match cmd {
            "forward" => (x + amt, y + aim * amt, aim),
            "down" => (x, y, aim + amt),
            "up" => (x, y, aim - amt),
            _ => (x, y, aim),
        }
    })
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
    assert_eq!(part1(input.clone()), 2117664);
    assert_eq!(part2(input), 2073416724);
}

