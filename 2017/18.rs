use advent::prelude::*;

use crate::CommandType::{ADD, JGZ, MOD, MUL, RCV, SET, SND};

#[derive(Copy, Clone)]
enum CommandType {
    SND,
    SET,
    ADD,
    MUL,
    MOD,
    RCV,
    JGZ,
}

#[derive(Copy, Clone)]
enum Argument {
    None,
    Register(char),
    Value(i64),
}

#[derive(Copy, Clone)]
struct Command {
    command_type: CommandType,
    arg1: Argument,
    arg2: Argument,
}

struct Program {
    commands: Vec<Command>,
    p: i64,
    index: i64,
    deadlock: bool,
    sends: i64,
    register: HashMap<char, i64>,
}

impl Program {
    fn new(
        commands: Vec<Command>,
        p: i64,
    ) -> Self {
        let mut register = HashMap::new();
        register.insert('p', p);
        Self {
            commands,
            p,
            index: 0,
            deadlock: false,
            sends: 0,
            register,
        }
    }

    fn value_of(&self, argument: Argument) -> i64 {
        match argument {
            Argument::None => 0,
            Argument::Register(r) => *self.register.get(&r).unwrap_or(&0i64),
            Argument::Value(v) => v,
        }
    }

    fn execute(&mut self, own: &mut VecDeque<i64>, other: &mut VecDeque<i64>) {
        if let Some(&command) = self.commands.try_get(self.index) {
            let arg1 = match command.arg1 {
                Argument::Register(c) => c
                _ => panic!("Expected a Register variant"),
            };
            if self.deadlock {
                self.rcv(command, own)
            } else {
                match command.command_type {
                    SND => {
                        other.push_back(self.value_of(command.arg1));
                        self.sends += 1;
                    }
                    SET => {
                        self.register.insert(arg1, self.value_of(command.arg2));
                    }
                    ADD => {
                        self.register.insert(
                            arg1,
                            self.value_of(command.arg1) + self.value_of(command.arg2),
                        );
                    }
                    MUL => {
                        self.register.insert(
                            arg1,
                            self.value_of(command.arg1) * self.value_of(command.arg2),
                        );
                    }
                    MOD => {
                        self.register.insert(
                            arg1,
                            self.value_of(command.arg1) % self.value_of(command.arg2),
                        );
                    }
                    RCV => {
                        self.rcv(command, own);
                        return;
                    }
                    JGZ => {
                        if self.value_of(command.arg1) > 0 {
                            self.index += self.value_of(command.arg2);
                            return;
                        }
                    }
                }
                self.index += 1;
            }
        } else {
            self.deadlock = true;
        }
    }

    fn rcv(&mut self, command: Command, own: &mut VecDeque<i64>) {
        if let Some(value) = own.pop_front() {
            let arg1 = match command.arg1 {
                Argument::Register(arg1) => arg1,
                _ => return
            };
            self.register.insert(arg1, value);
            self.index += 1;
            self.deadlock = false;
        } else {
            self.deadlock = true;
        }
    }
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let (command_type, remainder) = line.split_once(' ').unwrap();
            let command_type = match command_type {
                "snd" => SND,
                "set" => SET,
                "add" => ADD,
                "mul" => MUL,
                "mod" => MOD,
                "rcv" => RCV,
                "jgz" => JGZ,
                _ => panic!("Invalid input"),
            };
            if let Some((arg1, arg2)) = line.split_once(' ') {
                let arg1 = Argument::Register(arg1.chars().next().unwrap());
                if let Ok(value) = arg2.parse::<i64>() {
                    Command {
                        command_type,
                        arg1,
                        arg2: Argument::Value(value),
                    }
                } else {
                    Command {
                        command_type,
                        arg1,
                        arg2: Argument::Register(arg2.chars().next().unwrap()),
                    }
                }
            } else {
                let arg1 = Argument::Register(remainder.chars().next().unwrap());
                Command {
                    command_type,
                    arg1,
                    arg2: Argument::None,
                }
            }
        })
        .collect()
}

fn default_input() -> Vec<Command> {
    parse_input(include_input!(2017 / 18))
}

fn part1(input: Vec<Command>) -> i64 {
    todo!("part 1")
}

fn part2(input: Vec<Command>) -> i64 {
    todo!("part 2")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1);
    assert_eq!(part2(input), 2);
}
