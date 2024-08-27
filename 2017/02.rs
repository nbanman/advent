use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|line| get_numbers(line)).collect()
}

fn default_input() -> Vec<Vec<usize>> {
    parse_input(include_input!(2017 / 02))
}

fn solve<F>(spreadsheet: Vec<Vec<usize>>, line_operation: F) -> usize
    where
        F: Fn(Vec<usize>) -> usize,
{
    spreadsheet.into_iter().map(|row| line_operation(row)).sum()
}

fn part1(spreadsheet: Vec<Vec<usize>>) -> usize {
    solve(spreadsheet, |row| {
        let (min, max) = min_max(&row);
        max - min
    })
}

fn part2(spreadsheet: Vec<Vec<usize>>) -> usize {
    solve(spreadsheet, |row| {
        let mut sum = 0;
        for i in 0..row.len() - 1 {
            for j in &row[i + 1..row.len()] {
                let i = row[i];
                let bind = &[i, *j];
                let (smaller, larger) = min_max(bind);
                if larger % smaller == 0 {
                    sum += larger / smaller
                }
            }
        }
        sum
    })
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        "5 1 9 5
7 5 3
2 4 6 8",
    );
    assert_eq!(part1(input), 18)
}

#[test]
fn example2() {
    let input = parse_input(
        "5 9 2 8
9 4 7 3
3 8 6 5",
    );
    assert_eq!(part2(input), 9)
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 45972);
    assert_eq!(part2(input), 326);
}
