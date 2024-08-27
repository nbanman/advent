use std::collections::HashSet;
use std::ops::Range;

use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 03)
}

fn numbers_adjacent_to_symbol<F>(schematic: &str, width: usize, symbol: F) -> Vec<HashSet<Range<usize>>>
    where
        F: Fn(&char) -> bool
{
    schematic.chars().enumerate()
        .filter(|(_, c)| symbol(c))
        .map(|(index, _)| {
            let mut set_of_ranges: HashSet<Range<usize>> = HashSet::new();
            for y in -1isize..=1 {
                for x in -1isize..=1 {
                    let new_index = index as isize + y * width as isize + x;
                    if let Some(int_range) = get_number(schematic, new_index) {
                        set_of_ranges.insert(int_range);
                    }
                }
            };
            set_of_ranges
        }).collect()
}

fn get_number(schematic: &str, index: isize) -> Option<Range<usize>> {
    if index < 0 || index >= schematic.len() as isize { return None; };
    if !schematic.as_bytes()[index as usize].is_ascii_digit() { return None; };
    let mut left_index = index as usize;
    let mut right_index = index as usize;
    while let Some(x) = left_index.checked_sub(1) {
        if schematic.as_bytes()[x].is_ascii_digit() {
            left_index = x;
        } else {
            break;
        }
    };
    while schematic.as_bytes()[right_index + 1].is_ascii_digit() {
        right_index += 1;
    };
    Some(left_index..right_index)
}

fn part1(schematic: &str) -> usize {
    let width = schematic.find('\n').unwrap() + 1;
    let symbol = |c: &char| {
        *c != '\n' && *c != '.' && !c.is_ascii_digit()
    };
    numbers_adjacent_to_symbol(schematic, width, symbol)
        .iter()
        .flatten()
        .unique()
        .map(|range| {
            let start = range.start;
            let end = range.end;
            &schematic[start..=end]
        })
        .filter_map(|substring| substring.parse::<usize>().ok())
        .sum()
}

fn part2(schematic: &str) -> usize {
    let width = schematic.find('\n').unwrap() + 1;
    let symbol = |c: &char| {
        *c == '*'
    };
    numbers_adjacent_to_symbol(schematic, width, symbol)
        .iter()
        .filter(|set| set.len() == 2)
        .map(|set| {
            set
                .iter()
                .map(|range| {
                    let start = range.start;
                    let end = range.end;
                    &schematic[start..=end]
                })
                .filter_map(|substring| substring.parse::<usize>().ok())
                .product::<usize>()
        }).sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(part1(input), 4361);
    assert_eq!(part2(input), 467835);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 525911);
    assert_eq!(part2(input), 75805607);
}
