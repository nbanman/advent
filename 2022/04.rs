use advent::prelude::*;

fn parse_input(input: &str) -> Vec<[i32; 4]> {
    input
        .lines()
        .map(|line| {
            line
                .split(&[',', '-'])
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        }).collect()
}

fn default_input() -> Vec<[i32; 4]> {
    parse_input(include_input!(2022 / 04))
}

fn contains_all(low_1: &i32, high_1: &i32, low_2: &i32, high_2: &i32) -> bool {
    low_1 <= low_2 && high_1 >= high_2
}

fn overlaps(low_1: &i32, high_1: &i32, low_2: &i32, high_2: &i32) -> bool {
    if low_1 <= low_2 {
        high_1 >= low_2
    } else {
        high_2 >= low_1
    }
}


fn part1(pairs: Vec<[i32; 4]>) -> usize {
    pairs
        .iter()
        .filter(|[low_1, high_1, low_2, high_2]| {
            contains_all(low_1, high_1, low_2, high_2) || contains_all(low_2, high_2, low_1, high_1)
        })
        .count()
}

fn part2(pairs: Vec<[i32; 4]>) -> usize {
    pairs
        .into_iter()
        .filter(|[low_1, high_1, low_2, high_2]| {
            overlaps(low_1, high_1, low_2, high_2)
        })
        .count()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
    );
    assert_eq!(part1(input.clone()), 2);
    assert_eq!(part2(input), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 605);
    assert_eq!(part2(input), 914);
}
