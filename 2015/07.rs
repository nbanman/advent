use advent::prelude::*;
use crate::Instruction::{Assign, LShift, Not, RShift};
use crate::Value::{Signal, Wire};

#[derive(Clone, Debug)]
enum Value {
    Signal(u16),
    Wire(&'static str)
}

impl Value {
    fn new(s: &'static str) -> Self {
        if let Ok(signal) = s.parse() {
            Signal(signal)
        } else {
            Wire(s)
        }
    }

    fn value_of(&self, register: &HashMap<&'static str, u16>) -> Option<u16> {
        match self {
            Signal(value) => Some(*value),
            Wire(name) => { register.get(*name).copied() }
        }
    }

    fn name_of(&self) -> Option<&'static str> {
        match self {
            Signal(_) => None,
            Wire(name) => Some(*name),
        }
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Assign { from: Value, to: Value },
    LShift { wire: Value, amt: u16, to: Value },
    RShift { wire: Value, amt: u16, to: Value },
    Or { wire1: Value, wire2: Value, to: Value },
    Not { wire: Value, to: Value },
    And { value: Value, wire:Value, to: Value },
}

impl Instruction {
    fn run(&self, register: &mut HashMap<&'static str, u16>) -> bool {
        match self {
            Assign { from, to } => {
                let Some(from) = from.value_of(register) else { return false };
                register.insert(to.name_of().unwrap(), from);
            }
            LShift { wire, amt, to } => {
                let Some(from) = wire.value_of(register) else { return false };
                register.insert(to.name_of().unwrap(), from << amt);
            }
            RShift { wire, amt, to } => {
                let Some(from) = wire.value_of(register) else { return false };
                register.insert(to.name_of().unwrap(), from >> amt);
            }
            Instruction::Or { wire1, wire2, to } => {
                let Some(wire1) = wire1.value_of(register) else { return false };
                let Some(wire2) = wire2.value_of(register) else { return false };
                register.insert(to.name_of().unwrap(), wire1 | wire2);
            }
            Not { wire, to } => {
                let Some(from) = wire.value_of(register) else { return false };
                register.insert(to.name_of().unwrap(), from ^ 65535);
            }
            Instruction::And { value, wire, to } => {
                let Some(value) = value.value_of(register) else { return false };
                let Some(wire) = wire.value_of(register) else { return false };
                register.insert(to.name_of().unwrap(), value & wire);
            }
        }
        return true
    }

    fn to(&self) -> &'static str {
        let to = match self {
            Assign { from: _, to } => to,
            LShift { wire: _, amt: _, to } => to,
            RShift { wire: _, amt: _, to } => to,
            Instruction::Or { wire1: _, wire2: _, to } => to,
            Not { wire: _, to } => to,
            Instruction::And { value: _, wire: _, to } => to,
        };
        to.name_of().unwrap()
    }
}

fn parse_input(input: &'static str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let args: Vec<&str> = line.split(' ').collect();
            match args.len() {
                3 => Assign { from: Value::new(args[0]), to: Value::new(args[2]) },
                4 => Not { wire: Wire(args[1]), to: Wire(args[3]) },
                5 => {
                    match args[1] {
                        "RSHIFT" => {
                            RShift {
                                wire: Wire(args[0]),
                                amt: args[2].parse().unwrap(),
                                to: Wire(args[4]),
                            }
                        },
                        "LSHIFT" => {
                            LShift {
                                wire: Wire(args[0]),
                                amt: args[2].parse().unwrap(),
                                to: Wire(args[4]),
                            }
                        },
                        "OR" => {
                            Instruction::Or {
                                wire1: Wire(args[0]),
                                wire2: Wire(args[2]),
                                to: Wire(args[4]),
                            }
                        },
                        "AND" => {
                            Instruction::And {
                                value: Value::new(args[0]),
                                wire: Wire(args[2]),
                                to: Wire(args[4]),
                            }
                        },
                        _ => panic!("Invalied input! No command name given.")
                    }
                },
                _ => panic!("Invalid input! Incorrect number of arguments.")
            }
        })
        .collect()
}

fn default_input() -> Vec<Instruction> {
    parse_input(include_input!(2015 / 07))
}

fn solve(instructions: Vec<Instruction>, register: &mut HashMap<&'static str, u16>) -> u16 {
    let mut instructions = instructions.clone();
    let mut unexecuted_instructions = Vec::new();
    while !instructions.is_empty() {
        for instruction in instructions.into_iter() {
            if !instruction.run(register) { unexecuted_instructions.push(instruction) }
        }
        instructions = unexecuted_instructions.clone();
        unexecuted_instructions.clear();
    }
    *register.get("a").unwrap()
}

fn part1(input: Vec<Instruction>) -> u16 {
    let mut register = HashMap::new();
    solve(input, &mut register)
}

fn part2(input: Vec<Instruction>) -> u16 {
    let mut register = HashMap::new();
    let b = solve(input.clone(), &mut register);
    register.clear();
    register.insert("b", b);
    let input = input.into_iter()
        .filter(|instruction| {
            instruction.to() != "b"
        })
        .collect();
    solve(input, &mut register)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 46065);
    assert_eq!(part2(input), 14134);
}
