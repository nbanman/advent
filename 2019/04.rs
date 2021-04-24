use std::ops::Range;

use itertools::Itertools;

type Digits = [i8; 8];

fn default_input() -> Range<u64> {
    123257..647015
}

fn digits(mut num: u64) -> Digits {
    let mut digits = [-1, 0, 0, 0, 0, 0, 0, -1];
    for i in (1..7).rev() {
        digits[i] = (num % 10) as i8;
        num /= 10;
    }
    digits
}

fn has_increasing(digits: &Digits) -> bool {
    digits[1..7].iter().tuple_windows().all(|(x, y)| x <= y)
}

fn has_two_adjacent(digits: &Digits) -> bool {
    digits[1..7].iter().tuple_windows().any(|(x, y)| x == y)
}

fn has_two_adjacent_excl(digits: &Digits) -> bool {
    digits
        .iter()
        .tuple_windows()
        .any(|(w, x, y, z)| w != x && x == y && y != z)
}

fn part1(input: Range<u64>) -> usize {
    input
        .map(digits)
        .filter(|digits| has_increasing(&digits) && has_two_adjacent(&digits))
        .count()
}

fn part2(input: Range<u64>) -> usize {
    input
        .map(digits)
        .filter(|digits| has_increasing(&digits) && has_two_adjacent_excl(&digits))
        .count()
}

fn main() {
    let mut run = advent::start();
    let input = default_input();
    run.result("Part 1", part1(input.clone()));
    run.result("Part 2", part2(input));
    run.finish();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 2220);
    assert_eq!(part2(input), 1515);
}
