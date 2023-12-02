use advent::prelude::*;

fn parse_input(input: &str) -> Vec<u32> {
    let mut elves = Vec::new();
    let mut sum = 0;

    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(n) => sum += n,
            Err(_) => {
                elves.push(sum);
                sum = 0;
            }
        }
    }
    elves.push(sum);
    elves.sort_by(|a, b| b.cmp(a));
    elves
}

fn default_input() -> Vec<u32> {
    parse_input(include_input!(2022 / 01))
}

fn part1(input: Vec<u32>) -> u32 {
    input[0]
}

fn part2(input: Vec<u32>) -> u32 {
    input[0..3].iter().sum()
}

fn main() {
    let solution = advent::new(default_input)
        .part(part1)
        .part(part2)
        .build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
    );
    assert_eq!(part1(input.clone()), 24000);
    assert_eq!(part2(input), 45000);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 71300);
    assert_eq!(part2(input), 209691);
}
