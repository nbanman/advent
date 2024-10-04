use advent::prelude::*;

fn parse_input(input: &str) -> Vec<[usize; 26]> {
    let width = input.find('\n').unwrap();
    let mut columns = vec![[0usize; 26]; width];
    for line in input.lines() {
        for (index, c) in line.chars().enumerate() {
            columns[index][c as usize - 97] += 1;
        }
    }
    columns
}

fn default_input() -> Vec<[usize; 26]> {
    parse_input(include_input!(2016 / 06))
}

fn part1(columns: Vec<[usize; 26]>) -> String {
    columns.into_iter()
        .map(|column| {
            let m = column.into_iter().enumerate()
                .max_by_key(|(_, n)| *n)
                .unwrap()
                .0 as u8 + 97;
            m as char
        })
        .collect()
}

fn part2(columns: Vec<[usize; 26]>) -> String {
    columns.into_iter()
        .map(|column| {
            let m = column.into_iter().enumerate()
                .min_by_key(|(_, n)| *n)
                .unwrap()
                .0 as u8 + 97;
            m as char
        })
        .collect()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), "asvcbhvg".to_string());
    assert_eq!(part2(input), "odqnikqv".to_string());
}
