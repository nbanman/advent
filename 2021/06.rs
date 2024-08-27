use advent::prelude::*;

fn parse_input(input: &str) -> Vec<usize> { get_numbers(input) }

fn default_input() -> Vec<usize> {
    parse_input(include_input!(2021 / 06))
}

fn solve(fish: Vec<usize>, days: usize) -> usize {
    let mut fish_by_days = [0usize; 9];
    for age in fish {
        fish_by_days[age] += 1;
    }
    for _ in 0..days {
        fish_by_days.rotate_left(1);
        fish_by_days[6] += fish_by_days[8];
    }
    fish_by_days.into_iter().sum()
}

fn part1(fish: Vec<usize>) -> usize {
    solve(fish, 80)
}

fn part2(fish: Vec<usize>) -> usize {
    solve(fish, 256)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input("3,4,3,1,2");
    assert_eq!(solve(input.clone(), 18), 26);
    assert_eq!(part2(input), 26984457539);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 361169);
    assert_eq!(part2(input), 1634946868992);
}
