//! The latest and greatest intcode computer.

#![allow(dead_code)]

use std::cmp::max;

pub use super::intcode::*;

#[derive(Debug)]
pub struct Computer {
    mem: Vec<i64>,
    ptr: usize,
    relative_base: i64,
}

impl Computer {
    pub fn new(program: Vec<i64>) -> Self {
        Self {
            mem: program,
            ptr: 0,
            relative_base: 0,
        }
    }

    fn mem_get(&self, addr: usize) -> i64 {
        self.mem.get(addr).copied().unwrap_or(0)
    }

    fn mem_get_mut(&mut self, addr: usize) -> &mut i64 {
        self.mem.resize(max(self.mem.len(), addr + 1), 0);
        &mut self.mem[addr]
    }

    fn param_ptr(&self, i: usize) -> usize {
        let opcode = self.mem_get(self.ptr);
        let ptr = self.ptr + i;
        match opcode / (10i64.pow((1 + i) as u32)) % 10 {
            0 => cast(self.mem_get(ptr)),
            1 => ptr,
            2 => cast(self.relative_base + self.mem_get(ptr)),
            mode => panic!("unknown mode `{}`", mode),
        }
    }

    fn param(&self, i: usize) -> i64 {
        self.mem_get(self.param_ptr(i))
    }

    fn param_mut(&mut self, i: usize) -> &mut i64 {
        self.mem_get_mut(self.param_ptr(i))
    }

    pub fn next(&mut self, mut input: Option<i64>) -> State {
        loop {
            match self.mem_get(self.ptr) % 100 {
                1 => {
                    *self.param_mut(3) = self.param(1) + self.param(2);
                    self.ptr += 4;
                }
                2 => {
                    *self.param_mut(3) = self.param(1) * self.param(2);
                    self.ptr += 4;
                }
                3 => {
                    if let Some(input) = input.take() {
                        *self.param_mut(1) = input;
                        self.ptr += 2;
                    } else {
                        break State::Waiting;
                    }
                }
                4 => {
                    let output = self.param(1);
                    self.ptr += 2;
                    break State::Yielded(output);
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
                9 => {
                    self.relative_base += self.param(1);
                    self.ptr += 2;
                }
                99 => break State::Complete,
                opcode => panic!("unknown opcode `{}`", opcode),
            }
        }
    }

    pub fn next_value(&mut self, input: Option<i64>) -> Option<i64> {
        match self.next(input) {
            State::Yielded(v) => Some(v),
            State::Complete => None,
            state => panic!("unexpected state `{:?}`", state),
        }
    }
}