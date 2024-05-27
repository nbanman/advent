use std::usize;
use advent::prelude::*;

fn parse_input(input: &str) -> (usize, Vec<usize>) {
     let codes: Vec<usize> = input.lines()
         .map(|line| usize::from_str_radix(line, 2).unwrap())
         .collect();
    let bit_length = input.find('\n').unwrap();
    (bit_length, codes)
}

fn default_input() -> (usize, Vec<usize>) {
    parse_input(include_input!(2021 / 03))
}

fn bit(v: usize, pos: usize, bit_length: usize) -> usize {
    (v >> ((bit_length - 1) - pos)) & 1
}

fn find_rate(bit_length: usize, codes: &Vec<usize>, target: bool) -> usize {
    let target = if target {
        1
    } else {
        0
    };
    (0..bit_length).fold(0usize, |acc, pos| {
        let meets_target = codes.iter()
            .filter(|&code| {
                bit(*code, pos, bit_length) == target
            })
            .count();
        if meets_target * 2 >= codes.len() {
            acc + (1 << ((bit_length - 1) - pos))
        } else {
            acc
        }
    })
}

fn find_rating<F>(bit_length: usize, codes: &Vec<usize>, predicate: F) -> usize
where
    F: Fn(isize) -> bool,
{
    let mut codes = codes.clone();
    for pos in 0..bit_length {
        if codes.len() > 1 {
            let pos_sum: usize = codes.iter()
                .map(|code| bit(*code, pos, bit_length))
                .sum();
            let diff: isize = pos_sum as isize * 2 - codes.len() as isize;
            codes = codes.into_iter().filter(|code| {
                let bit = bit(*code, pos, bit_length);
                if predicate(diff) {
                    bit == 1
                } else {
                    bit == 0
                }
            }).collect();
        } else {
            break
        }
    }
    *codes.first().unwrap()
}
fn part1((bit_length, codes): (usize, Vec<usize>)) -> usize {
    let gamma = find_rate(bit_length, &codes, true);
    let epsilon = find_rate(bit_length, &codes, false);
    gamma * epsilon
}

fn part2((bit_length, codes): (usize, Vec<usize>)) -> usize {
    let gamma = find_rating(bit_length, &codes, |diff| diff >= 0);
    let epsilon = find_rating(bit_length, &codes, |diff| diff < 0);
    gamma * epsilon
}

fn main() {
    let solution = advent::new(default_input)
        .part(|i| part1(i))
        .part(|i| part2(i))
        .build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
    );
    assert_eq!(part1(input.clone()), 198);
    assert_eq!(part2(input), 230);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 3969000);
    assert_eq!(part2(input), 4267809);
}

// Part 1:                             (120.5 µs)
// 3969000
//
// Part 2:                             (31.50 µs)
// 4267809