//! Benchmark the intcode computer from 2019.
//!
//! See https://redd.it/egq9xn

use crate::intcode::{Computer, parse_program, State};

mod intcode;

#[inline(never)]
fn bench(program: Vec<i64>, input: &[i64]) -> i64 {
    let mut c = Computer::new(program.to_vec());
    c.feed(input.iter().copied());
    let mut output = Vec::new();
    loop {
        match c.next_state() {
            State::Yielded(v) => output.push(v),
            State::Complete => break,
            State::Waiting => unreachable!(),
        }
    }
    *output.last().unwrap()
}

fn main() {
    let sum_of_primes = parse_program(include_str!("bench/sum-of-primes.intcode"));
    let ackermann = parse_program(include_str!("bench/ackermann.intcode"));
    let isqrt = parse_program(include_str!("bench/isqrt.intcode"));
    let divmod = parse_program(include_str!("bench/divmod.intcode"));
    let factor = parse_program(include_str!("bench/factor.intcode"));
    let benches: [(&str, &[i64], &[i64]); 7] = [
        ("sum-of-primes/100_000", &sum_of_primes, &[100_000]),
        ("sum-of-primes/2_000_000", &sum_of_primes, &[2_000_000]),
        ("ackermann/3,6", &ackermann, &[3, 6]),
        ("isqrt", &isqrt, &[130]),
        ("divmod", &divmod, &[1024, 3]),
        ("factor/2147483647", &factor, &[2147483647]),
        ("factor/19201644899", &factor, &[19201644899]),
    ];
    let mut b = advent::new(|| ());
    for (name, program, input) in benches {
        b.named(name, move |_| bench(program.to_vec(), input));
    }
    let solution = b.build();
    solution.cli()
}
