use std::ops::Mul;
use advent::prelude::*;

fn parse_input(s: &str) -> (Vec<bool>, usize) {
    let map = s.as_bytes();
    let width = map.iter().position(|c| c == &b'\n').unwrap();
    let map = map
        .iter()
        .filter_map(|&c| {
            match c {
                b'.' => Some(false),
                b'#' => Some(true),
                _ => None,
            }
        })
        .collect();
    (map, width)
}

fn default_input() -> (Vec<bool>, usize) {
    parse_input(include_input!(2020 / 03))
}

fn solve(map: &Vec<bool>, width: usize, right: usize, down: usize) -> usize {
    let height = map.len() / width;
    (down..height).step_by(down).enumerate().fold(0, |acc, (idx, y)| {
        let x = ((idx + 1 ) * right) % width;
        if let Some(value) = map.get(y * width + x) {
            let modifier = if *value { 1 } else { 0 };
            acc + modifier
        } else {
            acc
        }
    })
}

fn part1((map, width): (Vec<bool>, usize)) -> usize {
    solve(&map, width, 3, 1)
}

fn part2((map, width): (Vec<bool>, usize)) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].into_iter()
        .map(|(right, down)| solve(&map, width, right, down))
        .fold(1, usize::mul)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input("\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#");
    assert_eq!(part1(input.clone()), 7);
    assert_eq!(part2(input), 336);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 294);
    assert_eq!(part2(input), 5774564250);
}
