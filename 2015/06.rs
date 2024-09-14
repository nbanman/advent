use advent::prelude::*;
use crate::Command::{On, Toggle};

#[derive(Copy, Clone, Debug)]
enum Command {
    On,
    Off,
    Toggle,
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    command: Command,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Instruction {
    fn new(line: &str) -> Self {
        let (first, remainder) = line.split_once(' ').unwrap();
        let command = if first == "toggle" {
            Toggle
        } else {
            let (second, _) = remainder.split_once(' ').unwrap();
            if second == "on" { On } else { Command::Off }
        };
        let Some((x1, y1, x2, y2)) = line.get_numbers().collect_tuple() else {
            panic!("Error parsing input");
        };
        Instruction { command, x1, y1, x2, y2 }
    }
}
fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::new).collect()
}

fn default_input() -> Vec<Instruction> {
    parse_input(include_input!(2015 / 06))
}

fn solve(instructions: Vec<Instruction>, operation: fn(&[i64], usize, &Command) -> i64) -> i64 {
    let mut lights = vec![0; 1_000_000];
    for instruction in instructions {
        for y in instruction.y1..=instruction.y2 {
            for x in instruction.x1..=instruction.x2 {
                let index = y * 1_000 + x;
                lights[index] = operation(&lights, index, &instruction.command)
            }
        }
    }
    lights.into_iter().sum()
}
fn part1(input: Vec<Instruction>) -> i64 {
    let operation: fn(&[i64], usize, &Command) -> i64 =
        |lights: &[i64], idx: usize, command: &Command| {
        match command {
            On => 1,
            Command::Off => 0,
            Toggle => (lights[idx] - 1).abs(),
        }
    };

    solve(input, operation)
}

fn part2(input: Vec<Instruction>) -> i64 {
    let operation: fn(&[i64], usize, &Command) -> i64 =
        |lights: &[i64], idx: usize, command: &Command| {
            match command {
                On => lights[idx] + 1,
                Command::Off => max(0, lights[idx] - 1),
                Toggle => lights[idx] + 2,
            }
        };

    solve(input, operation)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 569999);
    assert_eq!(part2(input), 17836115);
}
