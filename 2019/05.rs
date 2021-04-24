use crate::intcode::{cast, parse_program};

const INPUT: &str = include_str!("input/05.txt");

pub fn default_input() -> Vec<i64> {
    parse_program(INPUT)
}

#[derive(Debug)]
struct Computer {
    mem: Vec<i64>,
    ptr: usize,
}

impl Computer {
    fn new(program: Vec<i64>) -> Self {
        Self {
            mem: program,
            ptr: 0,
        }
    }

    fn param_ptr(&self, i: usize) -> usize {
        let opcode = self.mem[self.ptr];
        let ptr = self.ptr + i;
        match opcode / (10i64.pow((1 + i) as u32)) % 10 {
            0 => cast(self.mem[ptr]),
            1 => ptr,
            mode => panic!("unknown mode `{}`", mode),
        }
    }

    fn param(&self, i: usize) -> i64 {
        self.mem[self.param_ptr(i)]
    }

    fn param_mut(&mut self, i: usize) -> &mut i64 {
        let ptr = self.param_ptr(i);
        &mut self.mem.as_mut_slice()[ptr]
    }

    fn run(&mut self, input: i64) -> i64 {
        let mut output = None;
        loop {
            match self.mem[self.ptr] % 100 {
                1 => {
                    *self.param_mut(3) = self.param(1) + self.param(2);
                    self.ptr += 4;
                }
                2 => {
                    *self.param_mut(3) = self.param(1) * self.param(2);
                    self.ptr += 4;
                }
                3 => {
                    *self.param_mut(1) = input;
                    self.ptr += 2;
                }
                4 => {
                    output = Some(self.param(1));
                    self.ptr += 2;
                }
                5 => {
                    if self.param(1) != 0 {
                        self.ptr = cast(self.param(2));
                    } else {
                        self.ptr += 3;
                    }
                }
                6 => {
                    if self.param(1) == 0 {
                        self.ptr = cast(self.param(2));
                    } else {
                        self.ptr += 3;
                    }
                }
                7 => {
                    *self.param_mut(3) = (self.param(1) < self.param(2)) as i64;
                    self.ptr += 4;
                }
                8 => {
                    *self.param_mut(3) = (self.param(1) == self.param(2)) as i64;
                    self.ptr += 4;
                }
                99 => break,
                opcode => panic!("unknown opcode `{}`", opcode),
            }
        }
        output.unwrap()
    }
}

pub fn part1(input: &[i64]) -> i64 {
    Computer::new(input.to_vec()).run(1)
}

pub fn part2(input: &[i64]) -> i64 {
    Computer::new(input.to_vec()).run(5)
}