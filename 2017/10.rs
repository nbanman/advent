use std::ops::{Add, Mul};
use advent::prelude::*;

fn default_input() -> &'static str { include_input!(2017 / 10).trim_end() }

fn knot_hash<E: Clone>(ring: Vec<E>, lengths: &[usize], skip: usize) -> Vec<E> {
    lengths.iter().enumerate().fold(ring, |acc_ring, (index, &length)| {
        knot(&acc_ring, length, skip + index)
    })
}

fn knot<E: Clone>(ring: &[E], length: usize, skip: usize) -> Vec<E> {
    let mut result: Vec<_> = ring[..length].iter().cloned().rev().collect();
    result.extend_from_slice(&ring[length..]);
    let len = result.len();
    result.rotate_left((length + skip) % len);
    result
}

fn part1(input: &str) -> usize {
    let lengths: Vec<usize> = get_numbers(input);
    let ring: Vec<_> = (0usize..256).into_iter().collect();
    let mut hash = knot_hash(ring, &lengths, 0);
    let temp = (1..lengths.len()).reduce(usize::add).unwrap();
    let len = hash.len();
    hash.rotate_right((lengths.iter().sum::<usize>() + temp) % len);
    hash.into_iter().take(2).reduce(usize::mul).unwrap()
}

fn dense_hash(lengths: Vec<usize>) -> String {
    let ring: Vec<_> = (0usize..256).collect();
    let shift_sum = lengths.iter().sum::<usize>() * 64;
    let skip_sum = lengths.len() * 64;
    let lengths_len = lengths.len();
    let mut p2 = (0..64).fold(ring, |acc, i| {
        knot_hash(acc, &lengths, i * lengths_len)
    });
    let total_skips = (1..skip_sum).reduce(usize::add).unwrap();
    let p2_len = p2.len();

    p2.rotate_right((shift_sum + total_skips) % p2_len);
    p2.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, &x| acc ^ x))
        .map(|x| format!("{:02x}", x))
        .collect::<String>()
}

fn part2(input: &str) -> String {
    let lengths: Vec<_> = input.as_bytes()
        .iter()
        .chain([17, 31, 73, 47, 23].iter())
        .map(|&i| i as usize)
        .collect();
    dense_hash(lengths)
}


fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 23874);
    assert_eq!(part2(input), String::from("e1a65bfb5a5ce396025fab5528c25a87"));
}
