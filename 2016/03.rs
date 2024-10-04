use advent::prelude::*;

fn default_input() -> &'static str { include_input!(2016 / 03) }

fn is_valid(triangle: &[usize; 3]) -> bool {
    let sorted: Vec<_> = triangle.into_iter().sorted().collect();
    sorted[0] + sorted[1] > *sorted[2]
}

fn solve(numbers: impl Iterator<Item = usize>) -> usize {
    numbers
        .array_chunked()
        .filter(|triangle| is_valid(triangle))
        .count()
}

fn part1(input: &str) -> usize {
    solve(input.get_numbers())
}

fn part2(input: &str) -> usize {
    let numbers: Vec<usize> = input.get_numbers().collect();
    let mut rearranged = Vec::new();
    for x in 0..3 {
        for y in 0..numbers.len() / 3 {
            if let Some(next) = numbers.try_get(y * 3 + x) {
                rearranged.push(*next);
            }
        }
    }
    solve(rearranged.into_iter())
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1032);
    assert_eq!(part2(input), 1838);
}
